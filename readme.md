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

- A series of L1 batch transaction hash as command line arguments.
  ![Here's supposed to be image.](https://snipboard.io/JdqpsB.jpg)

## Program ouput

- Number of batched transactions.
- How much gas used by users.
- How much gas used by sequencer _(in other words how much gas users supposed to use)_.
- Difference between these numbers in percentages.

![Here's also supposed to be image. ](https://snipboard.io/3KIrGi.jpg)
