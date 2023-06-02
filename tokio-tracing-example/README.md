
## Introduction

Tokio tracing event API would be used to record the log events. Following macros are supported:
- tracing::info!
- tracing::warn!
- tracing::error!
- tracing::trace!
- tracing::event!
- tracing::debug!

Logs can be 
- **structured** - specified as key-value pair, with optional message-template field 
- **unstructured** - specified as plain text

**It is recommended to use structured logging should be used whenever possible**

Following attributes are recommended to be provided as argument to these macros

- **event_id** = \<int64\>
    - A durable numerical identifier, that can be used to differentiate logs from each other. 
    - Mapped to Part-B eventId
- **event_name** = \<string\>
    - Short event identifier that describes an event (E.g, PaymentCompleted, MouseClicked)
    - Mapped to Part-B name
- **target** = \<string\>
    - This is to categorize the logs in separate tables/buckets at backend. It can represent
        - Component/subsystem emitting the logs. (say payment-system, auth-system)
        - The log-stream/category this log belongs to. (say data-plane, control-plane, heartbeat)
    - Mapped to Part-A name in CS, env_name in Dgrep.
    - If missing, the default name would be "Log"
    - TODO to decide - By default, all the logs would go to "Log" table, unless configured as an Exporter option.

### Example for unstructured logs - NOT RECOMMENDED

```rust
  warn!( target: "payment-system", event_id = 10, event_name = "payment-failure", "Payment was not successful!");
```
  or 

```rust
  event!(target: "payment-system", Level::WARN, event_id = 10, event_name = "payment-failure", "Payment was not successful!");
```

### Example for structured logs:

```rust
  warn!(target: "auth-system", event_id = 20, event_name = "auth-failure", user_name = "ferris", user_email = "ferris@rust-lang.org");
```

### Example for structured logs with message-template:

```rust
  info!(target: "auth-system", event_id = 21, event_name = "auth-success", user_name = "ferris", user_email = "ferris@rust-lang.org",
      "Successful login for {{user_name}} and {{user_email}}" );
```

