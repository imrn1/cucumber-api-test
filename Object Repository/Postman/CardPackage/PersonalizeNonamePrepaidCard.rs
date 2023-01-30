<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PersonalizeNonamePrepaidCard</name>
   <tag></tag>
   <elementGuidId>e91ccebb-2d34-4f7c-87c1-43d00375a790</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;card_no\&quot;: \&quot;string\&quot;,\r\n  \&quot;account_number\&quot;: \&quot;string\&quot;,\r\n  \&quot;expiry_date\&quot;: \&quot;string\&quot;,\r\n  \&quot;cvv\&quot;: \&quot;string\&quot;,\r\n  \&quot;wallet_number\&quot;: \&quot;string\&quot;,\r\n  \&quot;national_id\&quot;: \&quot;string\&quot;,\r\n  \&quot;mobile_phone\&quot;: \&quot;string\&quot;,\r\n  \&quot;account_type\&quot;: 0\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}​/v1/cp/Issuing/PersonalizeNonamePrepaidCard</restUrl>
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
      <id>0f7ac71f-04d4-4548-9707-7ad2547be776</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
