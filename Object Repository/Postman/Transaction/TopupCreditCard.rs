<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TopupCreditCard</name>
   <tag></tag>
   <elementGuidId>b4bbdd1e-7a13-4681-8f1e-4ec1ce148483</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ext_transaction_id\&quot;: \&quot;${ext_transaction_id}\&quot;,\n  \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n  \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;,\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;wallet_number\&quot;: \&quot;${wallet_number}\&quot;,\n  \&quot;credit_card_post_type_id\&quot;: \&quot;${credit_card_post_type_id}\&quot;,\n  \&quot;source_type\&quot;: \&quot;${source_type}\&quot;,\n  \&quot;channel_type\&quot;: \&quot;${channel_type}\&quot;,\n  \&quot;hash_key\&quot;: \&quot;${hash_key}\&quot;\n}\n&quot;,
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
      <webElementGuid>54204edc-5983-4b9f-8e4f-3b2250ad1c12</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>ec3a773c-44d1-4220-9b2b-908cfe2827bf</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/TopupCreditCard</restUrl>
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
      <id>2ea90a14-8abf-455d-8688-55899597d9db</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>109e8304-8f0b-45bc-982c-1768dcce5722</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b8cc7341-ffb5-42db-a171-bea9b03d8db9</id>
      <masked>false</masked>
      <name>ext_transaction_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>08133c6d-3a5b-4a3b-9b9b-6fe4966d313c</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d16315c8-d0e7-4935-9e43-4804ace2ed0b</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f2260811-a3ec-4442-aeca-a29770b6fde9</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>60d58ba0-3828-4295-a9dd-09a8d9aa682a</id>
      <masked>false</masked>
      <name>wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>104fbfa7-bb0a-493a-acb0-e099fcb86438</id>
      <masked>false</masked>
      <name>credit_card_post_type_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>393ffa09-0412-46e8-94eb-30c3b0247aab</id>
      <masked>false</masked>
      <name>source_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>69475561-2426-469d-acb5-bda908248cc8</id>
      <masked>false</masked>
      <name>channel_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e6f0a2e7-4b29-49c4-952d-1a24e980c8fa</id>
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


println(&quot;TopupCreditCard request body: \n&quot;+ request.getBodyContent().getText())


if(WS.getResponseStatusCode(response) == 400) {
	WS.verifyResponseStatusCode(response, 400)
}
if(WS.getResponseStatusCode(response) == 200) {
	WS.verifyResponseStatusCode(response, 200)
}

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;TopupCreditCard response body\n&quot;+json.toString(4)); // Print it with specified indentation



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
