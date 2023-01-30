<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateBusinessBankAccount</name>
   <tag></tag>
   <elementGuidId>aaefb546-9e26-47ef-916e-7943973ad620</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;id\&quot;: \&quot;402656668034919437\&quot;,\r\n  \&quot;tenant_id\&quot;:5,\r\n  \&quot;business_account_id\&quot;: 0,\r\n  \&quot;bank_id\&quot;: 1,\r\n  \&quot;currency_code\&quot;: \&quot;try\&quot;,\r\n  \&quot;name\&quot;: \&quot;test2\&quot;,\r\n  \&quot;account_holder_name\&quot;: \&quot;test2\&quot;,\r\n  \&quot;iban\&quot;: \&quot;TR370006200042200006898137\&quot;,\r\n  \&quot;account_no\&quot;: \&quot;2819777777\&quot;,\r\n  \&quot;swift_code\&quot;: \&quot;321\&quot;,\r\n  \&quot;branch_code\&quot;: \&quot;321\&quot;,\r\n  \&quot;is_active\&quot;: false,\r\n  \&quot;is_deleted\&quot;: true,\r\n  \&quot;bank_name\&quot;: \&quot;tst\&quot;,\r\n  \&quot;created_date_utc\&quot;: \&quot;2022-05-27T13:51:16.638Z\&quot;,\r\n  \&quot;updated_date_utc\&quot;: \&quot;2022-05-27T13:51:16.639Z\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/UpdateBusinessBankAccount</restUrl>
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
      <id>2d1f8e75-a628-4604-97d4-8a48bba6dc21</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
