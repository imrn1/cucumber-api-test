<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>WithdrawalToBankInstant</name>
   <tag></tag>
   <elementGuidId>a119f3d5-cc4b-42cc-96e4-af4668ed24b8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;ext_transaction_id\&quot;: \&quot;12332888888\&quot;,\r\n  \&quot;receiver_account_number\&quot;: \&quot;account088\&quot;,\r\n  \&quot;receiver_wallet_number\&quot;: \&quot;276535231\&quot;,\r\n  \&quot;receiver_national_id\&quot;: \&quot;28195841536\&quot;,\r\n  \&quot;receiver_account_holder_name\&quot;: \&quot;esra aykut\&quot;,\r\n  \&quot;receiver_iban\&quot;: \&quot;TR880001200979800001011772\&quot;,\r\n  \&quot;description\&quot;: \&quot;WithdrawalToBankInstant\&quot;,\r\n  \&quot;amount\&quot;: 1,\r\n  \&quot;currency_code\&quot;: \&quot;TRY\&quot;,\r\n  \&quot;is_save_bank_account\&quot;: true,\r\n  \&quot;bank_account_name\&quot;: \&quot;maas\&quot;,\r\n  \&quot;hash_key\&quot;: \&quot;\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/WithdrawalToBankInstant</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>f01b2f6b-44a8-4d5f-b492-59b733f7b02f</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
