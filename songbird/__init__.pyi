class Driver:
    async def make_driver() -> None:
        ...
    async def connect(
        token: str,
        endpoint: str,
        session_id: str,
        guild_id: int,
        channel_id: int,
        user_id: int
    ) -> None:
        ...

class SongbirdError(Exception):
    ...

class CouldNotConnectToRTPError(SongbirdError):
    ...
