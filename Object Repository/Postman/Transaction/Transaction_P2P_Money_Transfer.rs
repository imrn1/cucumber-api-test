<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Transaction_P2P_Money_Transfer</name>
   <tag></tag>
   <elementGuidId>8dbe8927-c0ac-4ce6-933d-3f790864d1c5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;sender_account_number\&quot;: \&quot;ZYNP01\&quot;,\r\n  \&quot;sender_wallet_number\&quot;: \&quot;1324597082\&quot;,\r\n  \&quot;amount\&quot;: 1,\r\n  \&quot;currency_code\&quot;: \&quot;TRY\&quot;,\r\n  \&quot;description\&quot;: \&quot;P2P TRANSFER TESTI\&quot;,\r\n  \&quot;receiver_wallet_number\&quot;: \&quot;997316201\&quot;,\r\n  \&quot;hash_key\&quot;: \&quot;\&quot;,\r\n  \&quot;ext_transaction_id\&quot;:\&quot;123\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/PersonalToPersonalTransfer</restUrl>
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
      <id>9ff727ab-071e-4a97-8fd6-458915714411</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
