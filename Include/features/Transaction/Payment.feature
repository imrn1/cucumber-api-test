Feature: Payment servisi

  Scenario Outline: payment servisi
    Given Do payment with parameters <ext_transaction_id> <sender_account_number> <sender_wallet_number> <currency_code> <amount> <description> <receiver_wallet_number> <source_type> <channel_type> <hash_key> <earn> <burn>
    When send request to <url> with <method> method
    Then Verify <verifyKey> is <expectedValue>


    Examples: 
| ext_transaction_id | sender_account_number | sender_wallet_number | currency_code | amount | description   | receiver_wallet_number | earn | burn | source_type | channel_type | hash_key | url                     | method | verifyKey | expectedValue |
|          201030157 |                  0000 |            412492408 | TRY           |     15 | test_işlem    |             1685099926 |    0 |    0 |             |              |          | /v1/Transaction/Payment | POST   | status    |             0 |
