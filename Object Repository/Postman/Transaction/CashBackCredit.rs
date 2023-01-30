<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CashBackCredit</name>
   <tag></tag>
   <elementGuidId>ae9125ed-3e03-43f5-9109-1ee8978317d5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ext_transaction_id\&quot;:\&quot;${ext_transaction_id}\&quot;,\n  \&quot;sender_account_number\&quot;: \&quot;${sender_account_number}\&quot;,\n  \&quot;sender_wallet_number\&quot;: \&quot;${sender_wallet_number}\&quot;,\n  \&quot;receiver_wallet_number\&quot;: \&quot;${receiver_wallet_number}\&quot;,\n  \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n  \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;,\n  \&quot;description\&quot;: \&quot;${description}\&quot;,\n\n  \&quot;source_type\&quot;:\&quot;${source_type}\&quot;,\n  \&quot;channel_type\&quot;: \&quot;${channel_type}\&quot;,\n  \&quot;hash_key\&quot;: \&quot;${hash_key}\&quot;\n\n   \n }\n &quot;,
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
      <webElementGuid>7a23e074-e9b1-4ffc-8517-602a615642d0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>224fffc0-4be3-498f-8cab-30af8ae50c12</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/CashBackCredit</restUrl>
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
      <id>234b8e3d-908e-4d18-b7a2-12933f9ddb62</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>c5fc3f7b-7fb8-49de-9f42-3b2eba7abbea</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d5e38c33-b2ff-4740-8494-7fafcadb2a72</id>
      <masked>false</masked>
      <name>ext_transaction_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>04bc5313-ecd6-4f33-b9d3-b37f1273e7bf</id>
      <masked>false</masked>
      <name>sender_account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fdb39919-6dfd-4de2-89dc-85acd8e7ba78</id>
      <masked>false</masked>
      <name>sender_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f7849723-faf1-48cc-9484-37c1a11e6e62</id>
      <masked>false</masked>
      <name>receiver_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>252b1ab2-2fe2-4eb1-8c04-ea2c0c41386f</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d06272f2-dab2-4221-a5d8-d1871f03325b</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e4da59e3-f2bc-4319-9245-95d5c20fef38</id>
      <masked>false</masked>
      <name>description</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2ad6ff28-58dc-41c4-8ff6-4eff04dbb09d</id>
      <masked>false</masked>
      <name>source_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5e1e14a2-029e-4e97-ad2b-b9b6a77ba424</id>
      <masked>false</masked>
      <name>channel_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9ae8bbe7-3410-42b8-a8c2-372fa1b2d29f</id>
      <masked>false</masked>
      <name>hash_key</name>
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

GlobalVariable.responseObject = response
GlobalVariable.requestObject = request


println(&quot;CashBackCredit request body: \n&quot;+ request.getBodyContent().getText())


if(WS.getResponseStatusCode(response) == 400) {
	WS.verifyResponseStatusCode(response, 400)
}
if(WS.getResponseStatusCode(response) == 200) {
	WS.verifyResponseStatusCode(response, 200)
}

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;CashBackCredit response body\n&quot;+json.toString(4)); // Print it with specified indentation






</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
