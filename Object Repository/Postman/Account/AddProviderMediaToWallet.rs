<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddProviderMediaToWallet</name>
   <tag></tag>
   <elementGuidId>eef50818-8472-4c8c-a669-186cdc5ad323</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;wallet_number\&quot;: \&quot;${wallet_number}\&quot;,\n  \&quot;provider_id\&quot;: \&quot;${provider_id}\&quot;,\n  \&quot;media_key\&quot;:\&quot;${media_key}\&quot; \n}\n &quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>eb3756ee-2a2b-48df-8bd4-e20be1ffe549</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>b45088e4-a031-4fd8-be3f-0265beb29547</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/AddProviderMediaToWallet</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>226f7dc2-9757-45a4-914c-f26e21564c50</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>69c4b921-1f34-4374-983f-e3e2eb3bd58d</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.provider_id</defaultValue>
      <description></description>
      <id>0702da5e-bffb-4e89-85e2-d9379772cb83</id>
      <masked>false</masked>
      <name>provider_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.account_number</defaultValue>
      <description></description>
      <id>5efae504-30dc-42b9-8f51-b2f0a743d42f</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.wallet_number</defaultValue>
      <description></description>
      <id>8ae3f2b6-0afe-4b8a-a3fb-2bac52b0ca03</id>
      <masked>false</masked>
      <name>wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3386edea-8a25-46d5-9917-a35fe40f4170</id>
      <masked>false</masked>
      <name>media_key</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


println(&quot;AddProviderMediaToWallet request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyElementPropertyValue(response, 'status', 0)


WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)



import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;AddProviderMediaToWallet response body\n&quot;+json.toString(4)); // Print it with specified indentation

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
