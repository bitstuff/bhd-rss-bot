# bhd-rss-bot
RSS Monitoring Bot for BHD

Still a work-in-progress, and definitely several rough/sharp edges,
but core functionality seems to be working.

Simply put, files in a given RSS monitor are checked to ensure that
for some `match` in that `monitor`:

* the name matches `regex`
* `category` matches given `category`
* `resolution` is the same as specified
* `size` is less-than or equal to `max_size` in bytes

If a file matches all of the above, it is grabbed and placed in the
configured `dropdir`.

## Configuration

[TOML](https://toml.io/en/) config lives in
`${HOME}/.config/bhd-rss-bot/config.toml`,
looks something like

```
dropdir = "/tmp/foo"
frequency = 300

[[monitor]]
name = "Monitor 1"
url = "<RSS URL>"

  [[monitor.match]]
  name = "Example 1 (case-insensitive)"
  category = "Test"
  regex = "(?i)example.one"
  resolution = "1080p"
  max_size = 3221225000
  
  [[monitor.match]]
  name = "Example 2 (case-sensitive)"
  category = "Test2"
  regex = "Example.Two"
  resolution = "2160p"
  max_size = 3758096000

[[monitor]]
name = "Monitor 2"
url = "<Another RSS URL>"
frequency = 600

  [[monitor.match]]
  name = "Example 3 (case-insensitive)"
  category = "Test3"
  regex = "(?i)example.three"
  resolution = "720p"
  max_size = 2684355000
```

That is a directory to drop files into, the frequency at which to
poll, and an array of monitors.
Monitors consist of a name, a url, 
and an array of patterns to match against.
In this example the first monitor contains two matches, the
second only one.  Matches consist of a name, a
regular expression, a category, a resolution, and
a maximum file size.  

## Running in docker

The included Dockerfile seems to be working.  Build with

```
docker build -t bitstuff/bhd-rss-bot .
```

and run with something like

```
docker run -d \
  --restart unless-stopped \
  -v $HOME/.config/bhd-rss-bot:/.config/bhd-rss-bot:ro \
  -v <dropdir>:<dropdir> \
  --name bhd-rss-bot \
  bitstuff/bhd-rss-bot:latest

```
