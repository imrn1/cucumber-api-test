<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PostAuthorization</name>
   <tag></tag>
   <elementGuidId>7b45b171-47af-4533-bb70-98ee0c11047d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;tx_correlation_id\&quot;: 0,\r\n  \&quot;ext_transaction_id\&quot;: \&quot;string\&quot;,\r\n  \&quot;ext_transaction_id_ref\&quot;: \&quot;string\&quot;,\r\n  \&quot;amount\&quot;: 0,\r\n  \&quot;currency_code\&quot;: \&quot;string\&quot;,\r\n  \&quot;hash_key\&quot;: \&quot;string\&quot;,\r\n  \&quot;source_type\&quot;: \&quot;string\&quot;,\r\n  \&quot;channel_type\&quot;: \&quot;string\&quot;,\r\n  \&quot;media_identifier\&quot;: \&quot;string\&quot;,\r\n  \&quot;terminal_no\&quot;: \&quot;string\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/cp/Transaction/PostAuthorization</restUrl>
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
      <id>68ff6162-0770-4412-9ae9-9e71cae6ebc7</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
