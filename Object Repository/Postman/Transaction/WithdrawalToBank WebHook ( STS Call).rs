<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>WithdrawalToBank WebHook ( STS Call)</name>
   <tag></tag>
   <elementGuidId>1bf918d6-7f45-40b5-90b5-6bfa0aeaa4be</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;ext_transaction_id\&quot; : \&quot;12345645\&quot;,\r\n\&quot;tx_correlation_id\&quot; : \&quot;1769125922283842573\&quot;,\r\n\&quot;receiver_account_holder_name\&quot; :\&quot;Wallet User Name Surname\&quot;,\r\n\&quot;receiver_iban\&quot; :\&quot;TR_12345656576767\&quot;,\r\n\&quot;receiver_bank_account_number\&quot; :\&quot;TR_12345656576767\&quot;,\r\n\&quot;receiver_bank_code\&quot; :\&quot;ISBNK\&quot;,\r\n\&quot;receiver_bank_name\&quot; :\&quot;İş Bankası\&quot;,\r\n\&quot;receiver_bank_branch_code\&quot; :\&quot;\&quot;,\r\n\&quot;amount\&quot;: \&quot;23.45\&quot;,\r\n\&quot;currency_code\&quot;: \&quot;TRY\&quot;,\r\n\&quot;completed_date_utc\&quot; :\&quot;\&quot;,\r\n\&quot;wallet_info\&quot; : {\r\n\&quot;currency_code\&quot;: \&quot;TRY\&quot;,\r\n\&quot;account_number\&quot; : \&quot;XXX48400\&quot;,\r\n\&quot;number\&quot;: \&quot;1454952348\&quot;,\r\n\&quot;total_balance\&quot; : \&quot;501.00\&quot;,\r\n\&quot;payment_balance\&quot; : { \&quot;available\&quot; : \&quot;100.50\&quot;, \&quot;unavailble\&quot; : \&quot;150.00\&quot; },\r\n\&quot;cash_balance\&quot; : { \&quot;available\&quot; : \&quot;100.50\&quot;, \&quot;unavailble\&quot; : \&quot;150.00\&quot; },\r\n\&quot;transaction_limits\&quot; : {\r\n\&quot;max_balance\&quot; : \&quot;1250\&quot;,\r\n\&quot;topup_credit_limit\&quot; : \&quot;150.00\&quot;,\r\n\&quot;topup_cash_limit\&quot; : \&quot;150.00\&quot;,\r\n\&quot;withdrawal_limit\&quot; : \&quot;150.00\&quot;,\r\n\&quot;payment_limit\&quot; : \&quot;150.00\&quot;,\r\n\&quot;transaction_limit\&quot; : \&quot;150.00\&quot; }           \r\n\r\n}\r\n}\r\n&quot;,
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
