# Inversion of Control (IoC)

```
                 Inversion of Control (IoC)

       https://en.wikipedia.org/wiki/Inversion_of_control

            alice -> bank1 ->|     |-> bank2 -> bob

The Times 03/Jan/2009 Chancellor on brink of second bailout for banks

+---------------------------------------------------------------+

                    2009 Call me anytime

            alice  -----------1-------------->  bob

bitcoin-cli sendtoaddress 12c6DSiU4Rq3P4ZxziKxzrL5LmMBrzjrJX 1.68

+---------------------------------------------------------------+

              2018 Don't call me, I'll call you

+-----------------------+
|                       |
|    grin wallet send   |             +---------------------+
|     -d send.tx        |   send.tx   |                     |
|     -m file           +-----1------->                     |
|     1.68              |             |                     |
|                       |             |                     |
|                       |             | grin wallet receive |
|  +-----------------+  | send.tx.resp|   -i send.tx        |
|                       <-----2-------+                     |
|                       |             |                     |
|  grin wallet finalize |             |                     |
|    -i send.tx.resp    |             +----------^----------+
|                       |                        |
|                       +---------3--------------+
|                       |
+-----------------------+

```

# a free pass for the grin network.

銀河鐵道999 - 維基百科 https://zh.wikipedia.org/wiki/%E9%8A%80%E6%B2%B3%E9%90%B5%E9%81%93999

- [AVAILABLE] grin-happy-new-pig-year-169.json
- [USED] grin-happy-new-pig-year-168.json