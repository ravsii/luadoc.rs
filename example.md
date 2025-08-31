# Class TimerManager

## Methods

| Name | Params | Returns | Description |
|------|--------|---------|-------------|
| .setup | opts: `Config` |  |  |
| .start_timer | t: `Timer Timer object to start.` | integer id Timer ID, fun() cancel Cancel func that can be used to stop the timer. It | Starts a timer and tracks it in TimerManager.active_timers. The function returns 2 values for cancellation. already knows the correct ID. |
| .get_closest_timer |  | Timer? timer First timer that's about to expire or nil, if there are | no timers |
| .cancel | id: `integer` | boolean value true if the timer was found and stopped | Cancels a timer by its id |
| .cancel_all |  |  | Cancel all active timers |
| .setup_user_commands |  |  |  |
| .setup_autocmds |  |  |  |
| .save_state |  |  |  |
| .load_state |  |  |  |
| .active_timers_num |  | integer count Amount of active timers |  |

# Class Config

## Fields

| Name | Type | Description |
|------|------|-------------|
| persistent? | `boolean Save state across Neovim reloads.` |  |
| default_timer? | `TimerOpts Default values for new timers.` |  |

## Methods

| Name | Params | Returns | Description |
|------|--------|---------|-------------|
| .setup | opts: `Config` |  |  |

# Class Timer:TimerOpts

## Fields

| Name | Type | Description |
|------|------|-------------|
| created | `number  -- os.time()` |  |
| duration | `Duration` |  |

# Class TimerOpts

## Fields

| Name | Type | Description |
|------|------|-------------|
| message? | `string` | Message that shows up on timer finish. No effect, if on_start is passed. |
| icon? | `string | boolean` | Icon that will be passed to nvim.notify, false to don't pass anything |
| title? | `string` |  |
| log_level? | `vim.log.levels` |  |
| on_start? | `fun(t: Timer)` | Can be used to replace the default callback |
| on_finish? | `fun(t: Timer)` | Can be used to replace the default callback |

## Methods

| Name | Params | Returns | Description |
|------|--------|---------|-------------|
| .new | duration: `Duration|number If number, it's converted to Duration as ms.`, opts: `TimerOpts` | Timer | Create a new timer. @see TimerManager.start_timer starts it. |
| :remaining |  | Duration | Get remaining time in seconds |

# Class UI

## Methods

| Name | Params | Returns | Description |
|------|--------|---------|-------------|
| .active_timers |  |  | Shows the list of active timers |
| .cancel |  |  | Shows the list of active timers to cancel |
| .cancel_all |  |  | Copy of TimerManager.cancel_all, but also gives a feedback message, if there were any timers @see TimerManager.cancel_all |

# Class Duration

## Fields

| Name | Type | Description |
|------|------|-------------|
| value | `integer Duration in milliseconds` |  |

## Methods

| Name | Params | Returns | Description |
|------|--------|---------|-------------|
| .from | ms: `integer duration in milliseconds (default: 0)` | Duration | Create a new Duration object |
| :asMilliseconds |  | integer ms milliseconds, suitable for lua functions | Return value in milliseconds |
| :asSeconds |  | integer seconds | Return value in seconds |
| :sub | sub: `Duration` | Duration result | Returns a new Duration representing the result of subtracting `sub` from this duration. This does not modify the current Duration instance. |
| .parse_format | str: `string Go's time.Duration-like format` | Duration | Parse a duration string into a Duration object. Supports integer and fractional values, with units: - `s` for seconds - `m` for minutes - `h` for hours Values without unit are parsed as milliseconds. ### Examples ```lua local d1 = Duration.parse("3m")        -- 3 minutes → 180000 ms local d2 = Duration.parse("3.5m")      -- 3 min 30 sec → 210000 ms local d3 = Duration.parse("1.75h")     -- 1 hour 45 min → 6300000 ms local d4 = Duration.parse("45s")       -- 45 seconds → 45000 ms local d5 = Duration.parse("1500")      -- raw milliseconds → 1500 ms ``` |
| :into_hms |  | string? duration | Returns a human-readable duration string. Formats based on the duration length: - `hh:mm:ss` for durations of 1 hour or more - `mm:ss` / `m:ss` for durations between 1 minute and 1 hour. ss is a number - `xxs` / `xs` for durations less than 1 minute. x is a number and s represends seconds |

