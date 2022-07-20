# Batch decoder
Batch decoder for [Optimism Rollup](https://github.com/ethereum-optimism/optimism).
## How to setup
First things first, download the source code.
```shell
git clone https://github.com/SliceOfKekus/batch-decoder.git
```
Next step is setting environment variable. Set `MAINNET_URL` using any public network endpoints for mainnet in `.env.example` file and change it to `.env`.

And now it's ready to use!

## Program input
- L1 batch transaction hash.
![Here's supposed to be image.](https://snipboard.io/JdqpsB.jpg)

## Program ouput
- Number of batched transactions.
- How much gas used by users.
- How much gas used by sequencer *(in other words how much gas users supposed to use)*.
- Difference between these numbers in percentages.

![Here's also supposed to be image. ](https://snipboard.io/3KIrGi.jpg)
