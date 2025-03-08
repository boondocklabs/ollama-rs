use std::sync::{Arc, Mutex};

use tokio::io::{stdout, AsyncWriteExt};
use tokio_stream::StreamExt as _;

use crate::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponse, MessageRole},
        parameters::FormatType,
        tools::ToolGroup,
    },
    history::ChatHistory,
    models::ModelOptions,
    Ollama,
};

/// A coordinator for managing chat interactions and tool usage.
///
/// This struct is responsible for coordinating chat messages and tool
/// interactions within the Ollama service. It maintains the state of the
/// chat history, tools, and generation options.
pub struct Coordinator<C: ChatHistory, T: ToolGroup> {
    model: String,
    ollama: Ollama,
    options: ModelOptions,
    history: Arc<Mutex<C>>,
    tools: T,
    debug: bool,
    format: Option<FormatType>,
}

impl<C: ChatHistory> Coordinator<C, ()> {
    /// Creates a new `Coordinator` instance without tools.
    ///
    /// # Arguments
    ///
    /// * `ollama` - The Ollama client instance.
    /// * `model` - The model to be used for chat interactions.
    /// * `history` - The chat history manager.
    ///
    /// # Returns
    ///
    /// A new `Coordinator` instance.
    pub fn new(ollama: Ollama, model: String, history: C) -> Self {
        Self {
            model,
            ollama,
            options: ModelOptions::default(),
            history: Arc::new(Mutex::new(history)),
            tools: (),
            debug: false,
            format: None,
        }
    }
}

impl<C: ChatHistory, T: ToolGroup> Coordinator<C, T> {
    /// Creates a new `Coordinator` instance with tools.
    ///
    /// # Arguments
    ///
    /// * `ollama` - The Ollama client instance.
    /// * `model` - The model to be used for chat interactions.
    /// * `history` - The chat history manager.
    /// * `tools` - The tool group to be used.
    ///
    /// # Returns
    ///
    /// A new `Coordinator` instance with tools.
    pub fn new_with_tools(ollama: Ollama, model: String, history: C, tools: T) -> Self {
        Self {
            model,
            ollama,
            options: ModelOptions::default(),
            history: Arc::new(Mutex::new(history)),
            tools,
            debug: false,
            format: None,
        }
    }

    pub fn format(mut self, format: FormatType) -> Self {
        self.format = Some(format);
        self
    }

    pub fn options(mut self, options: ModelOptions) -> Self {
        self.options = options;
        self
    }

    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Get a handle to the history
    pub fn history(&self) -> Arc<Mutex<C>> {
        self.history.clone()
    }

    pub async fn chat(
        &mut self,
        messages: Vec<ChatMessage>,
    ) -> crate::error::Result<ChatMessageResponse> {
        if self.debug {
            for m in &messages {
                eprintln!("Hit {} with:", self.model);
                eprintln!("\t{:?}: '{}'", m.role, m.content);
            }
        }

        let mut request = ChatMessageRequest::new(self.model.clone(), messages)
            .options(self.options.clone())
            .tools::<T>();

        if let Some(format) = &self.format {
            let mut tools = vec![];
            T::tool_info(&mut tools);

            // If no tools are specified, set the format on the request. Otherwise wait for the
            // recursive call by checking that the last message in the history has a Tool role,
            // before setting the format. Ollama otherwise won't call the tool if the format
            // is set on the first request.
            if tools.is_empty() {
                request = request.format(format.clone());
            } else if let Some(last_message) = self
                .history
                .lock()
                .map_err(|_| crate::error::OllamaError::MutexPoisoned)?
                .messages()
                .last()
            {
                if last_message.role == MessageRole::Tool {
                    request = request.format(format.clone());
                }
            }
        }

        let mut stream = self
            .ollama
            .send_chat_messages_with_history_stream(self.history.clone(), request)
            .await?;

        let mut response = String::new();
        while let Some(Ok(res)) = stream.next().await {
            stdout().write_all(res.message.content.as_bytes()).await?;
            stdout().flush().await?;
            response += res.message.content.as_str();
        }

        Ok(ChatMessageResponse {
            model: self.model.clone(),
            created_at: String::default(),
            message: ChatMessage::assistant(response),
            done: true,
            final_data: None,
        })

        /*
        let resp = self
            .ollama
            .send_chat_messages_with_history(&mut self.history, request)
            .await?;
        */

        /*
        if !resp.message.tool_calls.is_empty() {
            for call in resp.message.tool_calls {
                if self.debug {
                    eprintln!("Tool call: {:?}", call.function);
                }

                let resp = self.tools.call(&call.function).await?;

                if self.debug {
                    eprintln!("Tool response: {}", &resp);
                }

                self.history.push(ChatMessage::tool(resp))
            }

            // recurse
            Box::pin(self.chat(vec![])).await
        } else {
            if self.debug {
                eprintln!(
                    "Response from {} of type {:?}: '{}'",
                    resp.model, resp.message.role, resp.message.content
                );
            }

            Ok(resp)
        }
        */
    }
}
