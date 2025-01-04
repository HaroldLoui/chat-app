### 接口调用返回数据结构如下：

```json
{
  "id": "chatcmpl-123",
  "object": "chat.completion",
  "created": 1677652288,
  "model": "gpt-4o-mini",
  "system_fingerprint": "fp_44709d6fcb",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "你好，请问有什么可以帮助你的吗？"
      },
      "logprobs": null,
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 9,
    "completion_tokens": 12,
    "total_tokens": 21,
    "completion_tokens_details": {
      "reasoning_tokens": 0,
      "accepted_prediction_tokens": 0,
      "rejected_prediction_tokens": 0
    }
  }
}
```

### 流式调用返回数据结构如下：

```txt
data: {"id":"chatcmpl-AlRwE8zNJkTxhGi9miACDRLyaOPXi","object":"chat.completion.chunk","created":1735872174,"model":"gpt-4o-mini-2024-07-18","system_fingerprint":"fp_d02d531b47","choices":[{"index":0,"delta":{"role":"assistant","content":"","refusal":null},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-AlRwE8zNJkTxhGi9miACDRLyaOPXi","object":"chat.completion.chunk","created":1735872174,"model":"gpt-4o-mini-2024-07-18","system_fingerprint":"fp_d02d531b47","choices":[{"index":0,"delta":{"content":"你好"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-AlRwE8zNJkTxhGi9miACDRLyaOPXi","object":"chat.completion.chunk","created":1735872174,"model":"gpt-4o-mini-2024-07-18","system_fingerprint":"fp_d02d531b47","choices":[{"index":0,"delta":{"content":"！"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-AlRwE8zNJkTxhGi9miACDRLyaOPXi","object":"chat.completion.chunk","created":1735872174,"model":"gpt-4o-mini-2024-07-18","system_fingerprint":"fp_d02d531b47","choices":[{"index":0,"delta":{"content":"有什么"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-AlRwE8zNJkTxhGi9miACDRLyaOPXi","object":"chat.completion.chunk","created":1735872174,"model":"gpt-4o-mini-2024-07-18","system_fingerprint":"fp_d02d531b47","choices":[{"index":0,"delta":{"content":"我"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-AlRwE8zNJkTxhGi9miACDRLyaOPXi","object":"chat.completion.chunk","created":1735872174,"model":"gpt-4o-mini-2024-07-18","system_fingerprint":"fp_d02d531b47","choices":[{"index":0,"delta":{"content":"可以"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-AlRwE8zNJkTxhGi9miACDRLyaOPXi","object":"chat.completion.chunk","created":1735872174,"model":"gpt-4o-mini-2024-07-18","system_fingerprint":"fp_d02d531b47","choices":[{"index":0,"delta":{"content":"帮助"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-AlRwE8zNJkTxhGi9miACDRLyaOPXi","object":"chat.completion.chunk","created":1735872174,"model":"gpt-4o-mini-2024-07-18","system_fingerprint":"fp_d02d531b47","choices":[{"index":0,"delta":{"content":"你"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-AlRwE8zNJkTxhGi9miACDRLyaOPXi","object":"chat.completion.chunk","created":1735872174,"model":"gpt-4o-mini-2024-07-18","system_fingerprint":"fp_d02d531b47","choices":[{"index":0,"delta":{"content":"的吗"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-AlRwE8zNJkTxhGi9miACDRLyaOPXi","object":"chat.completion.chunk","created":1735872174,"model":"gpt-4o-mini-2024-07-18","system_fingerprint":"fp_d02d531b47","choices":[{"index":0,"delta":{"content":"？"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-AlRwE8zNJkTxhGi9miACDRLyaOPXi","object":"chat.completion.chunk","created":1735872174,"model":"gpt-4o-mini-2024-07-18","system_fingerprint":"fp_d02d531b47","choices":[{"index":0,"delta":{},"logprobs":null,"finish_reason":"stop"}]}

data: [DONE]
```