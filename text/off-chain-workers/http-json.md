# HTTP Fetching and JSON Parsing in Off-chain Workers

`pallets/ocw-demo`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%ocw-demo%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

## HTTP Fetching

In traditional web apps, we use HTTP requests to communicate with and fetch data from third-party APIs.
But this is tricky when we want to perform this in Substrate runtime on chain. First, HTTP requests
are indeterministic. There are uncertainty in terms of how long the request will take, and the result
may not be the same all the time. This causes problem for the network reaching consensus.

So in Substrate runtime, we use off-chain workers to issue HTTP requests and fetching the results back.

In this chapter, we will dive into fetching data using GitHub RESTful API on the organization `substrate-developer-hub`
that hosts this recipe.

We issue an http request and return the JSON string in byte vector inside the `fetch_from_remote()`
function.

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
fn fetch_from_remote() -> Result<Vec<u8>, Error<T>> {
	// Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
	let request = rt_offchain::http::Request::get(HTTP_REMOTE_REQUEST);

	// Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
	let timeout = sp_io::offchain::timestamp()
		.add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD));

	// For github API request, we also need to specify `user-agent` in http request header.
	//   See: https://developer.github.com/v3/#user-agent-required
	let pending = request
		.add_header("User-Agent", HTTP_HEADER_USER_AGENT)
		.deadline(timeout) // Setting the timeout time
		.send() // Sending the request out by the host
		.map_err(|_| <Error<T>>::HttpFetchingError)?;

	// By default, the http request is async from the runtime perspective. So we are asking the
	//   runtime to wait here.
	// The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
	//   ref: https://substrate.dev/rustdocs/v3.0.0/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
	let response = pending
		.try_wait(timeout)
		.map_err(|_| <Error<T>>::HttpFetchingError)?
		.map_err(|_| <Error<T>>::HttpFetchingError)?;

	if response.code != 200 {
		debug::error!("Unexpected http request status code: {}", response.code);
		return Err(<Error<T>>::HttpFetchingError);
	}

	// Next we fully read the response body and collect it to a vector of bytes.
	Ok(response.body().collect::<Vec<u8>>())
}
```

On the above code, we first create a request object `request`, and set a timeout period so the http request does not hold
indefinitely with `.deadline(timeout)`. For querying github APIs, we also need to add an extra HTTP
header of `user-agent` with `add_header(...)`. HTTP requests from off-chain workers are fetched asynchronously. Here
we use `try_wait()` to wait for the result to come back, and terminate and return if any errors occured, i.e. returning non-200 http response code.

Finally we get the response back from `response.body()`
iterator. Since we are in a `no_std` environment, we collect them back as a byte vector instead of a string and return.

## JSON Parsing

We frequently get data back in JSON format when requesting from HTTP APIs. The next task is to parse the JSON
and fetch the required key-value pairs out. This is demonstrated in the `fetch_n_parse` function.

### Setup

In Rust, `serde` and `serde-json` are the popular combo-package used for JSON parsing.

src: `pallets/ocw-demo/Cargo.toml`

```toml
#--snip--

[dependencies]
#--snip--

serde = { version = '1.0.100', default-features = false, features = ['derive'] }
serde_json = { version = '1.0.45', default-features = false, features = ['alloc'] }


#--snip--
```

### Deserializing JSON string to struct

Then we use the usual `serde-derive` approach on deserializing. First we define the struct with
fields that we are interested to extract out.

src:
`pallets/ocw-demo/src/lib.rs`

```rust
use serde::{Deserialize, Deserializer};

// ref: https://serde.rs/container-attrs.html#crate
#[derive(Deserialize, Encode, Decode, Default)]
struct GithubInfo {
	// Specify our own deserializing function to convert JSON string to vector of bytes
	#[serde(deserialize_with = "de_string_to_bytes")]
	login: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	blog: Vec<u8>,
	public_repos: u32,
}
```

By default, `serde` deserializes JSON strings to the `String` datatype. We want to write our own
deserializer to convert it to vector of bytes.

```rust
pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where D: Deserializer<'de> {
	let s: &str = Deserialize::deserialize(de)?;
	Ok(s.as_bytes().to_vec())
}
```

Now the actual deserialization takes place in the `fetch_n_parse` function.

```rust
/// Fetch from remote and deserialize the JSON to a struct
fn fetch_n_parse() -> Result<GithubInfo, Error<T>> {
	let resp_bytes = Self::fetch_from_remote()
		.map_err(|e| {
			debug::error!("fetch_from_remote error: {:?}", e);
			<Error<T>>::HttpFetchingError
		})?;

	let resp_str = str::from_utf8(&resp_bytes)
		.map_err(|_| <Error<T>>::HttpFetchingError)?;

	// Deserializing JSON to struct, thanks to `serde` and `serde_derive`
	let gh_info: GithubInfo = serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::HttpFetchingError)?;
	Ok(gh_info)
}
```

## Conclusion

In this chapter, we go over how to construct an HTTP request and send it out to the
GitHub API remote endpoint. We then demonstrate how to use `serde` library to
parse the JSON string we retrieved in the HTTP response into a data structure that
we can further manipulate in our runtime.
