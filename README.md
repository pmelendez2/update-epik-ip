
<h2>Motivation</h2>
An internet search to understand **\"What is Vibe Coding\?\"** was the motivation, for this personal project.

My domain provider is [Epik](https://www.epik.com) and on
occasion I had to login and update my domain IP address. 
Less often these days.  It was an annonyance for me but since
I updated Domain IP address manually, and infrequently it
was not a driving issue for me.  Until now, given a) I am 
learning Rust, and b) I wanted to understand in pratical terms
"Vibe Coding".

So after searching and reading a few articles on
"Vibe Coding".  I went to [Grok AI](https://x.ai), as I 
previously used Grok for some specific application
 configuration.  I thought it did a fair job.

<h2>Prompt</h2>

The prompt I entered in Grok was

> I would like to generate a program to updatei
> my Epik.com domain IP address when it changes.
> I would like the program writting in Rust, using
> Epik.com APIs, using my API Key.  The program should
> run on Ubuntu 25.04 from the command line.  The program
> will be run on a scheduled basis using cron.

<h2>Prompt Response </h2>

  To create a Rust program that updates the IP address of
 a domain registered with Epik.com using their API, we 
need to consider the following:

<h2>Epik API</h2>
The Epik API requires a valid signature and IP 
whitelisting for authentication. The API supports
DNS management, which includes updating the IP address of
a domain. The documentation is available at 
[Epik API Documentation](https://docs.userapi.epik.com/v2/).

<h2>Rust Program</h2>
The program will use the reqwest crate
for HTTP requests, serde for JSON serialization/deserialization,
and tokio for async operations. It will fetch the current
public IP and update the domain's A record via the Epik API.

<h2>Ubuntu 25.04</h2>
The program will be a command-line application, compiled 
to run on Ubuntu 25.04.

<h2>Cron Scheduling</h2>
The program will be designed to run
periodically via a cron job, checking for IP changes and
updating the domain as needed.

<h2>API Key</h2>
The user must provide their API key and ensure
their IP is whitelisted in the Epik account settings.

Below is a Rust program that accomplishes this task. 
The program assumes you have your Epik API key and domain
details. It fetches the current public IP using a service
like ipify.org and updates the domain's A record if the
IP has changed. The program stores the last known IP in a
local file to avoid unnecessary API 
calls.update_epik_ip.rs

<h2>Setup Instructions</h2>

Install Rust on Ubuntu 25.04

<h3>Install rustup if not already installed</h3>

```
bash sudo apt update
sudo apt install rustup 
rustup default stable
```

<h3>Install dependencies</h3>

```
bash sudo apt install build-essential pkg-config libssl-dev

```

<h2>Create a Rust Project</h2>

<h3>Create a new Rust project</h3>

```
bash cargo new epik-ip-updater
cd epik-ip-updater
```

Replace src/main.rs with the provided code.

<h3>Update Cargo.toml to include dependencies</h3>

```
toml[package]
name = "epik-ip-updater"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
sha2 = "0.10"
hex = "0.4"
chrono = "0.4"
```

<h3>Configure the Program</h3>

Replace YOUR_EPIK_API_KEY with your actual Epik API key.
Set domain to your domain (e.g., example.com).
Set subdomain if updating a subdomain (e.g., sub for 
sub.example.com); leave empty for the root domain.
Ensure your server's IP is whitelisted in your Epik
 account (see Epik API documentation).


<h3>Compile and Test</h3>
<h3>Build the program</h3>

```
    bashcargo build --release
```

<h3>Run the program</h3>

```
  bash ./target/release/epik-ip-updater
```

<h2>Set Up Cron Job</h2>

<h3>Open the crontab editor </h3>

```
bashcrontab -e
```

<h3>Add a cron job to run every 5 minutes</h3>

```
bash*/5 * * * * /path/to/epik-ip-updater/target/release/epik-ip-updater >> /var/log/epik-ip-updater.log 2>&1
```

<h3>Ensure the log file has appropriate permissions</h3>

```
bash sudo touch /var/log/epik-ip-updater.log
sudo chmod 664 /var/log/epik-ip-updater.log
```


<h2>Notes</h2>

<h3>API Key Security</h3>
Store your API key securely, preferably in an environment
 variable or a configuration file with restricted 
permissions, and load it at runtime instead of 
hardcoding it.
<h3>IP Whitelisting</h3>
Ensure your server's IP is whitelisted in your Epik account,
as required by the Epik API.
<h3>Error Handling</h3> The program checks for IP changes
to avoid unnecessary API calls. It logs errors to stderr 
and successes to stdout, which cron will capture in the 
log file.
<h3>Dependencies</h3> The program uses reqwest for HTTP
 requests, serde for JSON handling, sha2 and hex for
 signature generation, and chrono for timestamps.
<h3>Cron Frequency</h3> Adjust the cron schedule 
(e.g., */5 * * * * for every 5 minutes) based on how
 frequently your IP changes. Some ISPs change IPs every 12
 hours or less.
<h3>Epik API Documentation</h3> Refer to 
[Epik API Doc](https://docs.userapi.epik.com/v2/) for 
detailed API specifications, especially for the DNS update
 endpoint.

Per Grok this program should efficiently update your Epik
 domain's IP address when run via cron on Ubuntu 25.04.

<h3>Initial checkin is generated code.</h3>

<h3>Todos</h3>

After I received the generated Rust code, a quick review I
I listed following items Todo.

1. Setup Environmental variables to read API keys, domain name to update.
2. Pass command line parameters for API keys, and domain name to update.

