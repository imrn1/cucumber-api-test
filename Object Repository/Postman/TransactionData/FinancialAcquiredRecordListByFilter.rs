<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>FinancialAcquiredRecordListByFilter</name>
   <tag></tag>
   <elementGuidId>fb5f9d28-5118-46aa-8187-c924f481f58b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n\r\n//   \&quot;process_level_status_id\&quot;: 0,\r\n//   \&quot;to_wallet_id\&quot;: 0,\r\n//   \&quot;currency_code\&quot;: [\r\n//     \&quot;string\&quot;\r\n//   ],\r\n//   \&quot;from_account_number\&quot;: \&quot;string\&quot;,\r\n//   \&quot;from_account_type_Id\&quot;: 0,\r\n//   \&quot;from_account_Id\&quot;: 0,\r\n\r\n//    \&quot;from_wallet_number\&quot;: \&quot;string\&quot;,\r\n // \&quot;to_account_type_Id\&quot;: 2,\r\n //  \&quot;to_account_Id\&quot;: 22,\r\n  //\&quot;to_account_number\&quot;: null,\r\n//\&quot;to_wallet_number\&quot;: \&quot;1011179271\&quot;,\r\n //   \&quot;description\&quot;: \&quot;string\&quot;,\r\n\r\n\r\n \r\n\r\n // \&quot;tenant_id\&quot;:7,\r\n//   \&quot;wallet_id\&quot;: 0,\r\n//   \&quot;transaction_type_codes\&quot;: [\r\n//     0\r\n//   ],\r\n//   \&quot;start_date\&quot;: \&quot;2022-03-31T09:50:31.160Z\&quot;,\r\n //  \&quot;end_date\&quot;: \&quot;2022-01-01\&quot;,\r\n\r\n  \&quot;page_size\&quot;: 0,\r\n  \&quot;page_index\&quot;: 0,\r\n  \&quot;order_column\&quot;: \&quot;Id\&quot;,\r\n  \&quot;order_by\&quot;: \&quot;asc\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/TransactionData/FinancialAcquiredRecordListByFilter</restUrl>
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
      <id>85639c5c-ff4a-486d-9734-8f4c64bf8f9c</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
