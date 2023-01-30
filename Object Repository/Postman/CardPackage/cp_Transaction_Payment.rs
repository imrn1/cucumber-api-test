<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>cp_Transaction_Payment</name>
   <tag></tag>
   <elementGuidId>b0e9cd80-ba45-4b77-b3de-2b521f050329</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;sender_account_number\&quot;: \&quot;${sender_account_number}\&quot;,\n  \&quot;sender_wallet_number\&quot;: \&quot;${sender_wallet_number}\&quot;,\n  \&quot;ext_transaction_id\&quot;: \&quot;${ext_transaction_id}\&quot;,\n  \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;,\n  \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n  \&quot;description\&quot;: \&quot;${description}\&quot;,\n  \&quot;receiver_wallet_id\&quot;: \&quot;${receiver_wallet_id}\&quot;,  \n  \&quot;media_identifier\&quot;: \&quot;${media_identifier}\&quot;,\n  \&quot;source_type\&quot;: \&quot;${source_type}\&quot;,\n  \&quot;channel_type\&quot;: \&quot;${channel_type}\&quot;,\n  \&quot;hash_key\&quot;: \&quot;${hash_key}\&quot;,\n  \&quot;terminal_no\&quot;: \&quot;${terminal_no}\&quot;\n}&quot;,
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
      <webElementGuid>4e27bb1e-6672-49d1-982f-1c99afeada44</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>2aa47c8e-364f-41d2-906c-368afbeda9d5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/cp/Transaction/Payment</restUrl>
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
      <id>4e3038f4-71ea-4b6d-8bec-94b54f82eef6</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>44875a13-792f-447e-9036-92b3186a46f6</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f7e49bb5-c868-4b0b-9b1d-7443968d159d</id>
      <masked>false</masked>
      <name>ext_transaction_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>759571dc-d44e-4557-a79f-e45e50336e58</id>
      <masked>false</masked>
      <name>sender_account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0af207f1-64fb-4ea3-8c49-50be1ce113c1</id>
      <masked>false</masked>
      <name>sender_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>875301ee-aae0-47b1-900b-13513d3f2826</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ff37af8c-7bc0-416b-b5a1-422de077b32f</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3be0e88c-276e-41a5-84d2-4dc9072d86ec</id>
      <masked>false</masked>
      <name>description</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ddfe336f-11b7-4b88-8908-8b99c0286a31</id>
      <masked>false</masked>
      <name>receiver_wallet_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b11f4548-c62a-4daf-9c84-76cde4323d36</id>
      <masked>false</masked>
      <name>media_identifier</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bdbf5faa-333b-47ad-84d9-62b3c9a0898c</id>
      <masked>false</masked>
      <name>source_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>95e0b0fb-52ce-43ad-a4c1-97b77ba363ba</id>
      <masked>false</masked>
      <name>channel_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>00aaa3ed-ceb8-41a8-a0dc-5accca7a3ca2</id>
      <masked>false</masked>
      <name>hash_key</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>377949a5-c09b-42ca-af27-d58977e00b13</id>
      <masked>false</masked>
      <name>terminal_no</name>
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
 
println(&quot;cp/Transaction/Payment request body: \n&quot;+ request.getBodyContent().getText())

 
GlobalVariable.responseObject = response
GlobalVariable.requestObject = request


if(WS.getResponseStatusCode(response) == 400) {
	WS.verifyResponseStatusCode(response, 400)
}
if(WS.getResponseStatusCode(response) == 200) {
	WS.verifyResponseStatusCode(response, 200)
}

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;cp/Transaction/Payment response body\n&quot;+json.toString(4)); // Print it with specified indentation
 
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
