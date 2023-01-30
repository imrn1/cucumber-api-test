<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TopupCash Internal (Not For STS Call)</name>
   <tag></tag>
   <elementGuidId>525aabab-216f-4794-b8dc-74226badbe10</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n   \&quot;tenant_code\&quot;: \&quot;OTW\&quot;,\r\n   \&quot;sender_name\&quot;: \&quot;imran aykut\&quot;,\r\n   \&quot;sender_iban\&quot;: \&quot;TR0900111000000000894111111\&quot;,\r\n   \&quot;sender_bank_code\&quot;: \&quot;134\&quot;,\r\n   \&quot;sender_bank_name\&quot;: \&quot;DENIZBANK\&quot;,\r\n   \&quot;receiver_bank_code\&quot;: \&quot;134\&quot;,\r\n   \&quot;receiver_bank_name\&quot;: \&quot;DENIZBANK\&quot;,\r\n   \&quot;receiver_iban\&quot;: \&quot;TR330013400000588594111111\&quot;,\r\n   \&quot;amount\&quot;: 100.0,\r\n   \&quot;currency_code\&quot;: \&quot;TRY\&quot;,\r\n   \&quot;description\&quot;: \&quot;0101\&quot;,\r\n   \&quot;regex_definition_result_data\&quot;:\&quot;0101\&quot;,\r\n   \&quot;bank_transaction_date\&quot;:\&quot;2021-06-29 08:42\&quot;,\r\n   \&quot;bank_receipt_no\&quot;:\&quot;7582684222ff4252\&quot;,\r\n   \&quot;national_id_or_tax_no\&quot;: \&quot;28198841472\&quot; \r\n}\r\n\r\n\r\n\r\n\r\n\r\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/TopupCash</restUrl>
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
      <id>6445dd2e-1f33-4e09-bd51-f8404d50bc08</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
