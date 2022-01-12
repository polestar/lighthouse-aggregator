# Lighthouse Aggregator

CLI tool that make it easier to perform multiple lighthouse runs towards a single target and output the result
in a "plotable" format.

> note: [lighthouse](https://github.com/GoogleChrome/lighthouse#using-the-node-cli) must be installed and accessible in PATH

## Build

Building from source requires `rust >=1.56.0` to be installed.

1. make
2. make install

## Example

1. run the command `lighthouse-aggregator https://www.google.se -t -c 3 -o aggregate.json`
2. read the result `cat aggregate.json`

> All values are in ms.

```json
{
  "bootupTime": [205.19600000000008, 257.8040000000001, 89.75200000000002],
  "firstContentfulPaint": [1976.5049999999997, 1966.764, 1093.381],
  "firstMeaningfulPaint": [1976.5049999999997, 1966.764, 1138.881],
  "roundTripTime": [15.752, 16.485000000000003, 16.289],
  "serverResponseTime": [94.12, 87.027, 110.454],
  "timeStamp": "2022-01-11_16:52:43",
  "timeToInteractive": [2491.549, 2976.8045, 1152.381],
  "totalBlockingTime": [15.0, 15.0, 0.0],
  "totalByteWeight": [198761.0, 198845.0, 198842.0]
}
```
