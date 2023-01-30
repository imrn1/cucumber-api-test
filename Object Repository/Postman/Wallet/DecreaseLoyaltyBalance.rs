<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DecreaseLoyaltyBalance</name>
   <tag></tag>
   <elementGuidId>ce96ad67-ed86-4401-b789-3b72e88fb28f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;wallet_number\&quot;: \&quot;LYT_4888510684539251725\&quot;,\r\n  \&quot;amount\&quot;: 100,\r\n  \&quot;currency_code\&quot;: \&quot;TRY\&quot;,\r\n  \&quot;hash_key\&quot;: null\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Wallet/DecreaseLoyaltyBalance</restUrl>
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
      <id>f8bd0e86-4be1-405c-9ff0-2cc1a314a85b</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
