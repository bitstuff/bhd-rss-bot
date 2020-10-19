# bhd-rss-bot
RSS Monitoring Bot for BHD

[TOML](https://toml.io/en/) config lives in
`${HOME}/.config/bhd-rss-bot/config.toml`,
looks something like

```
dropdir = "/tmp/foo"

[[monitor]]
name = "Monitor 1"
url = "<RSS URL>"
frequency = 300

  [[monitor.matches]]
  name = "Example 1 (case-insensitive)"
  category = "Test"
  regex = "(?i)example.one"
  resolution = "1080p"
  max_size = 3221225000
  
  [[monitor.matches]]
  name = "Example 2 (case-sensitive)"
  regex = "Example.Two"
  resolution = "2160p"
  max_size = 3758096000

[[monitor]]
name = "Monitor 2"
url = "<Another RSS URL>"
frequency = 600

  [[monitor.matches]]
  name = "Example 3 (case-insensitive)"
  regex = "(?i)example.three"
  max_size = 2684355000
```

That is a directory to drop files into, and an array of monitors.
Monitors consist of a name, a url, the frequency at which to poll
the url for changes, and an array of patterns to match against.
In this example the first monitor contains two matches, the
second only one.  Matches consist of a name, a
regular expression, an optional category, an optional resolution, and
a maximum file size.  
