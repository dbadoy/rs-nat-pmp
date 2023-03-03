// UDP
const PORT: u16 = 5351;

// rfc-6886: (3.3) Requesting a Mapping
// 
// The RECOMMENDED Port Mapping Lifetime is 7200 seconds
// (two hours).
const DEFAULT_MAPPING_LIFETIME_SECOND: u32 = 7200; 

// rfc-6886: (3.1) Requests and Responses
//
// Performs a total of 9 retries per request, with an
// initial wait of 250ms and a doubling of the wait for
// each retry (64 seconds total).
const RETRY: u16 = 9;
const WAIT_TIME_MS: u16 = 250;
