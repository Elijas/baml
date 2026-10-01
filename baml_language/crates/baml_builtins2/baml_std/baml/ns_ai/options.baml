/// Optional LLM client deadlines. Null/non-positive values are unbounded.
class LlmClientOptions {
    /// Total deadline for the HTTP request, including a streaming response body.
    request_timeout: root.time.Duration?,
    /// Deadline for establishing the connection (including TLS).
    connect_timeout: root.time.Duration?,
    /// Streaming only: deadline from the response opening until the first SSE
    /// event carrying message content (text, reasoning, or a tool call). A
    /// protocol preamble event such as `message_start` does not satisfy it.
    time_to_first_token_timeout: root.time.Duration?,
    /// Streaming only: the longest allowed gap between SSE events.
    idle_timeout: root.time.Duration?,

    function new() -> LlmClientOptions throws never {
        LlmClientOptions {
            request_timeout: null,
            connect_timeout: null,
            time_to_first_token_timeout: null,
            idle_timeout: null,
        }
    }

    function with_request_timeout(
        self,
        request_timeout: root.time.Duration?,
    ) -> LlmClientOptions throws never {
        self.request_timeout = request_timeout;
        self
    }

    function with_connect_timeout(
        self,
        connect_timeout: root.time.Duration?,
    ) -> LlmClientOptions throws never {
        self.connect_timeout = connect_timeout;
        self
    }

    function with_time_to_first_token_timeout(
        self,
        time_to_first_token_timeout: root.time.Duration?,
    ) -> LlmClientOptions throws never {
        self.time_to_first_token_timeout = time_to_first_token_timeout;
        self
    }

    function with_idle_timeout(
        self,
        idle_timeout: root.time.Duration?,
    ) -> LlmClientOptions throws never {
        self.idle_timeout = idle_timeout;
        self
    }
}
