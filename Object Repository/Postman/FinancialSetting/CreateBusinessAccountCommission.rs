<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateBusinessAccountCommission</name>
   <tag></tag>
   <elementGuidId>5ea33621-bcb2-4a94-b415-46b9630a96f9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n \&quot;business_account_id\&quot;: 1,\r\n \&quot;txowner_kyc_level_code\&quot;: 10,\r\n \&quot;currency_code\&quot;:\&quot;TRY\&quot;,\r\n \&quot;transaction_type_code\&quot;: 3003,\r\n \&quot;business_account_commission_rate\&quot;: 2 ,\r\n \&quot;business_account_commission_fixed_amount\&quot;: 2,\r\n \&quot;end_user_commission_rate\&quot;: 1 ,\r\n \&quot;end_user_commission_fixedamount\&quot;: 1 ,\r\n \&quot;min_amount\&quot;:1,\r\n \&quot;max_amount\&quot;:100,\r\n \&quot;settlement_day\&quot;: 1\r\n}\r\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}​/v1/FinancialSetting/CreateBusinessAccountCommission</restUrl>
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
      <id>a9d260f7-6a93-4de2-96d7-9857d18d1616</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
