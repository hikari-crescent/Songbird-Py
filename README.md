# Songbird-Py
Songbird bindings for python. The goal is to provide an easy to use alternitive to Lavalink.
Its written with rust-bindings to [Songbird](https://github.com/serenity-rs/songbird).

### Playing a Song
Once you are connected to a channel, playing music is extremely easy.

```python
from songbird import ytdl

# `voice` was created from a connection to the gateway.

track_handle = await voice.play_source(ytdl("https://www.youtube.com/watch?v=r25MAkzkTF4"))

await sleep(5)
# Doesn't need to be awaited!
track_handle.pause()
await sleep(5)
track_handle.play()
```

### Supported Libraries
Hikari is currently the only supported library. See the examples directory for more information.

### Using with your own Gateway
```python
from asyncio import run
from songbird import Driver

async def main():
    voice = await Driver.create()
    # `server` is the server payload from the gateway.
    # `state` is the voice state payload from the gateway.
    await voice.connect(
        token=server.token,
        endpoint=server.endpoint,
        session_id=state.session_id,
        guild_id=server.guild_id,
        channel_id=state.channel_id,
        user_id=state.user_id
    )

run(main())
```

# Contributing
Pyo3 asyncio is used with tokio.

## Dependencies
[Maturin](https://github.com/PyO3/maturin) should be installed though pip. This is used to build the Rust code to a python lib.
Run command `maturin develop` when changes are made to the Rust src.

[pyo3](https://github.com/PyO3/pyo3)

[pyo3 docs](https://pyo3.rs/v0.15.1/)

[pyo3 asyncio](https://github.com/awestlake87/pyo3-asyncio)

[pyo3 asyncio docs](https://docs.rs/pyo3-asyncio/0.13.4/pyo3_asyncio/) You can also look at the async secion of the pyo3 docs.

### Songbird
[Link](https://github.com/serenity-rs/songbird)

[docs](https://serenity-rs.github.io/songbird/current/songbird/index.html)

Its a good idea to install all the dependencies.

## What should you run to test your changes?
I recommend working off of the `hikari` example. Make sure to run `maturin develop` when anything is changed!

## Goal of the project
Create API for songbird [driver](https://serenity-rs.github.io/songbird/current/songbird/driver/struct.Driver.html) and everything that is needed with it it.
