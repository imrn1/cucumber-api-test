<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PersonalToBusinessTransfer</name>
   <tag></tag>
   <elementGuidId>5d90b4a8-1d08-4306-9dd9-fdb71ce4a1dd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ext_transaction_id\&quot;: \&quot;${ext_transaction_id}\&quot;,\n  \&quot;sender_account_number\&quot;: \&quot;${sender_account_number}\&quot;,\n  \&quot;sender_wallet_number\&quot;: \&quot;${sender_wallet_number}\&quot;,\n  \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n  \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;,\n  \&quot;description\&quot;: \&quot;${description}\&quot;,\n  \&quot;receiver_wallet_number\&quot;: \&quot;${receiver_wallet_number}\&quot;,\n  \&quot;hash_key\&quot;: \&quot;${hash_key}\&quot;\n}\n &quot;,
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
      <webElementGuid>a9ec823e-94db-4bc9-ab3d-8fbcbf2371ec</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>b79f8cfb-3716-4856-96af-ad7e4421ecd3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/PersonalToBusinessTransfer</restUrl>
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
      <id>35e9ec53-3808-4918-a826-a4829208cbac</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>7bbdbcb9-5525-4698-920d-5d33f0def76d</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>325dc8fc-9816-4c50-b747-75939e5f4ba9</id>
      <masked>false</masked>
      <name>ext_transaction_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>da9dadde-3c0d-4e08-92d7-6c98409607e5</id>
      <masked>false</masked>
      <name>sender_account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b0d42772-75cc-4fe2-8fa8-3764f022ef2f</id>
      <masked>false</masked>
      <name>sender_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5908fc3e-9c95-4bee-be73-e01706da489d</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7a55729e-fd68-489d-acfc-b89ffcee2c1d</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b42e7156-0582-4228-9703-cd645e811715</id>
      <masked>false</masked>
      <name>description</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>849134a9-3feb-4af5-aa2b-cbc5f1959255</id>
      <masked>false</masked>
      <name>receiver_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b2cecdbd-0e86-4a49-a448-eb12c3c5713c</id>
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

println(&quot;PersonalToBusinessTransfer request body: \n&quot;+ request.getBodyContent().getText())


if(WS.getResponseStatusCode(response) == 400) {
	WS.verifyResponseStatusCode(response, 400)
}
if(WS.getResponseStatusCode(response) == 200) {
	WS.verifyResponseStatusCode(response, 200)
}

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText());
println(&quot;PersonalToBusinessTransfer response body\n&quot;+json.toString(4));










</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
