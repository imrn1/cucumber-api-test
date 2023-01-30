<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SendMoneyToBankAccount</name>
   <tag></tag>
   <elementGuidId>382345ce-f441-4150-98d5-9cc4ce1430ef</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n \&quot;ext_transaction_id\&quot;: \&quot;789647823347889\&quot;,\r\n  \&quot;sender_account_number\&quot;: \&quot;DNM002\&quot;,\r\n  \&quot;sender_wallet_number\&quot;: \&quot;534201820\&quot;,\r\n  \&quot;receiver_national_id_or_tax_no\&quot;: \&quot;123456\&quot;,\r\n\r\n\&quot;receiver_account_holder_name\&quot;: \&quot;test\&quot;,\r\n  \&quot;receiver_iban\&quot;: \&quot;AT483200000012345864\&quot;,\r\n  \&quot;description\&quot;: null,\r\n\r\n \&quot;amount\&quot;: 1,\r\n  \&quot;currency_code\&quot;: \&quot;TRY\&quot;,\r\n \r\n\r\n  \&quot;hash_key\&quot;: null\r\n}\r\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/SendMoneyToBankAccount</restUrl>
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
      <id>a3f7c814-8d40-4421-902f-9e0cbb72952f</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
