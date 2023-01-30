<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateWalletForPersonalAccount</name>
   <tag></tag>
   <elementGuidId>8b7d9f97-a2a2-4b8e-a535-93ababad79d7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;name\&quot;: \&quot;testwallet2\&quot;,\r\n  \&quot;account_number\&quot;: \&quot;2819\&quot;,\r\n  \&quot;currency_code\&quot;: \&quot;TRY\&quot;,\r\n  \&quot;use_iban\&quot;: false,\r\n  \&quot;tenant_id\&quot;:5\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Wallet/CreateWalletForPersonalAccount</restUrl>
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
      <id>5b3992cf-a1e5-4265-b896-e969acb00291</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
