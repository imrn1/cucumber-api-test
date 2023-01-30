<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TopupCash WebHook (STS Call)</name>
   <tag></tag>
   <elementGuidId>58aa65d2-68e5-4eba-b0e3-5131ca2ee781</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \r\n   \&quot;tx_correlation_id\&quot;:\&quot;12345677767\&quot;,\r\n   \&quot;sender_name\&quot;: \&quot;BITEXEN SENDER\&quot;,\r\n   \&quot;sender_iban\&quot;: \&quot;TRBTXNXXXXXXX\&quot;,\r\n   \&quot;sender_bank_code\&quot;: \&quot;99\&quot;,\r\n   \&quot;sender_bank_name\&quot;: \&quot;AKBANK\&quot;,\r\n   \&quot;receiver_bank_code\&quot;: \&quot;64\&quot;,\r\n   \&quot;receiver_bank_name\&quot;: \&quot;GARANTI\&quot;,\r\n   \&quot;receiver_iban\&quot;: \&quot;TRXXXXXXXXXXXXXXX\&quot;,\r\n   \&quot;amount\&quot;: 10.0,\r\n   \&quot;currency_code\&quot;: \&quot;TRY\&quot;,\r\n   \&quot;description\&quot;: \&quot;BTXN\&quot;,\r\n   \&quot;regex_definition_result_data\&quot;: \&quot;BTXN202105171222\&quot;,\r\n   \&quot;bank_transaction_date\&quot;:\&quot;2021-05-10 15:42\&quot;,\r\n   \&quot;bank_receipt_no\&quot;:\&quot;112233445566B778899A\&quot;,\r\n   \&quot;national_id_or_tax_no\&quot;: \&quot;2XXX9XX17X\&quot;,\r\n   \&quot;wallet_info\&quot;: {\r\n                \&quot;account_number\&quot;: \&quot;WLT48400\&quot;,\r\n                \&quot;number\&quot;: \&quot;306563917\&quot;,\r\n                \&quot;total_balance\&quot;: 0.00,\r\n                \&quot;payment_balance\&quot;: {\r\n                    \&quot;avaiable\&quot;: 0.00,\r\n                    \&quot;unavaiable\&quot;: 0.00\r\n                },\r\n                \&quot;cash_balance\&quot;: {\r\n                    \&quot;avaiable\&quot;: 0.00,\r\n                    \&quot;unavaiable\&quot;: 0.00\r\n                },\r\n                \&quot;transaction_limits\&quot;: {\r\n                    \&quot;max_balance\&quot;: 1250.0000,\r\n                    \&quot;topup_credit_limit\&quot;: 1250.0000,\r\n                    \&quot;topup_cash_limit\&quot;: 1250.0000,\r\n                    \&quot;withdrawal_limit\&quot;: 1250.0000,\r\n                    \&quot;payment_limit\&quot;: 1250.0000,\r\n                    \&quot;wallet_to_wallet_limit\&quot;: 1250.0000\r\n                },\r\n                \&quot;currency_code\&quot;: \&quot;TRY\&quot;\r\n},\r\n   \&quot;hash_key\&quot;: \&quot;\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://TenantSide.Whitelabel.endpoint.url</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
