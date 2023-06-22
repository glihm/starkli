# transaction-receipt subcommand

Get transaction receipt by hash.

```bash
$ starkli transaction-receipt 0x06a9f49148992175694e5bb5a34a352d775059117fcf987d4478f7d0f729860c

{
  "type": "INVOKE",
  "transaction_hash": "0x6a9f49148992175694e5bb5a34a352d775059117fcf987d4478f7d0f729860c",
  "actual_fee": "0x362a42e8830",
  "status": "ACCEPTED_ON_L2",
  "block_hash": "0x2527a93dcfdd42988d2668436c8901543dcb988ec6a8888038b9aebd57f6c1c",
  "block_number": 821985,
  "messages_sent": [],
  "events": [
    {
      "from_address": "0x259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478",
      "keys": [
        "0x3e6786b59c4ea963504194850298c5c97a60f5889515ccf4ac1845f225b7aa0"
      ],
      "data": [
        "0x737461726b6c69",
        "0x737461726b6e6574"
      ]
    },
    {
      "from_address": "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
      "keys": [
        "0x99cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9"
      ],
      "data": [
        "0x136b3bdee77cfcb343a2f30ee08fd5340fc83688b3514bc89bea4011fad720f",
        "0x1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8",
        "0x362a42e8830",
        "0x0"
      ]
    }
  ]
}
```
