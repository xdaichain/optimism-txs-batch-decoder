# Batch decoder
Batch decoder for optimism rollup.
## How to setup
First things first, download the source code.
```shell
git clone https://github.com/SliceOfKekus/batch-decoder.git
```
Next step is setting environment variables. Set `MAINNET_URL` and `OPTIMISM_URL` using any public network endpoints for mainnet and optimism respectively in `.env.example` file and change it to `.env`.

And now it's ready to use!

## Program input
- L1 transaction hash of batch.
![Here's supposed to be image.](https://snipboard.io/JdqpsB.jpg)

## Program ouput
- Number of batched transactions.
- How much users paid.
- Cost of batch submission *(in other words how much users supposed to paid)*.
- Difference between these to numbers in percents.

![Here's also supposed to be image. ](https://snipboard.io/A6D4Tv.jpg)
