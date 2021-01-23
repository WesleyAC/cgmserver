# `cgmserver`

A very lightweight reimplementation of part of the [Nightscout](https://github.com/nightscout/cgm-remote-monitor) REST API, designed to read data from [Xdrip+](https://github.com/NightscoutFoundation/xDrip), write it to a SQLite database, and provide an API to get recent blood sugar measurements. Specifically, it implements the [`POST /api/v1/entries`](https://github.com/nightscout/cgm-remote-monitor/blob/f2fd923261e396d774e233d61421ce7ed705524e/swagger.yaml#L345) endpoint, and a custom `/recents` endpoint which shows the last 45 minutes of data.

It should probably be run behind nginx or another reverse proxy which implements checking for the `API-Secret` header.
