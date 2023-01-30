<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Transaction_P2B_Payment</name>
   <tag></tag>
   <elementGuidId>b680dbd5-ea25-457f-bf91-7b8f011719d3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n\&quot;sender_account_number\&quot; : \&quot;SPY123456\&quot;,\r\n\&quot;sender_wallet_number\&quot;: \&quot;SPY123456WALTRY\&quot;,\r\n\&quot;ext_order_id\&quot; : \&quot;ABC123132\&quot; ,\r\n\&quot;business_code\&quot;: \&quot;1234512981\&quot;,\r\n\&quot;currency_code\&quot;: \&quot;TRY\&quot;,\r\n\&quot;amount\&quot;: \&quot;100\&quot;,\r\n\r\n\&quot;description\&quot;: \&quot;ürün bedeli\&quot;,\r\n\&quot;receiver_wallet_number\&quot;: \&quot;SPY123456WALTRY\&quot;,\r\n\&quot;hash_key\&quot;: \&quot;\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/Payment</restUrl>
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
      <id>dbc9017d-e5f9-4aaf-867c-6c40f76c9c0f</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
