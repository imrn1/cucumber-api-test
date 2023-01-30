<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetCardLimitAndRestriction</name>
   <tag></tag>
   <elementGuidId>cb0ab9f3-b97d-4b82-895e-0440b289d500</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;wallet_number\&quot;: \&quot;${wallet_number}\&quot;,\n  \&quot;card_no\&quot;: \&quot;${card_no}\&quot;,\n  \&quot;auth_ecom\&quot;: \&quot;${auth_ecom}\&quot;,\n  \&quot;auth_moto\&quot;: \&quot;${auth_moto}\&quot;,\n  \&quot;auth_contactless\&quot;: \&quot;${auth_contactless}\&quot;,\n  \&quot;auth_int\&quot;: \&quot;${auth_int}\&quot;,\n  \&quot;set_limit\&quot;: \&quot;${set_limit}\&quot;,\n  \&quot;daily_max_amount\&quot;: \&quot;${daily_max_amount}\&quot;,\n  \&quot;weekly_max_amount\&quot;: \&quot;${weekly_max_amount}\&quot;,\n  \&quot;monthly_max_amount\&quot;: \&quot;${monthly_max_amount}\&quot;,\n  \&quot;yearly_max_amount\&quot;: \&quot;${yearly_max_amount}\&quot;\n}&quot;,
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
      <webElementGuid>687d32a1-2c2b-4c7f-87ab-da62792aba5b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>d2c98db8-0288-4977-9b71-bc1d0b6db922</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1​/cp​/Issuing​/SetCardLimitAndRestriction</restUrl>
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
      <id>96557900-ce55-4b91-87cf-710a76943332</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>7bd1f19c-469f-4259-90c8-a62de684f3fd</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3e958275-a960-4a8d-b912-3e5e99d36a6e</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d03086f9-09de-43c5-b5ab-a4c6f880cce6</id>
      <masked>false</masked>
      <name>wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8fe21ef5-e704-4d6b-b196-1b09e47a3426</id>
      <masked>false</masked>
      <name>card_no</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2496ad96-360b-43f0-b5b2-ebfc43cbb745</id>
      <masked>false</masked>
      <name>auth_ecom</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ed5a4e59-b23a-4178-917b-6f109fe2a76b</id>
      <masked>false</masked>
      <name>auth_moto</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>da3a7a6f-7be9-40df-a11a-172067431348</id>
      <masked>false</masked>
      <name>auth_contactless</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ed5143e2-428b-43cc-bd6c-ead9927ffc66</id>
      <masked>false</masked>
      <name>auth_int</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e8fa174e-8459-4d33-9173-e39361944dac</id>
      <masked>false</masked>
      <name>set_limit</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8a2dcb78-33ed-4662-9286-50b45de4a04a</id>
      <masked>false</masked>
      <name>daily_max_amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9723d205-d7b0-4739-b9ae-12f0ae57e44d</id>
      <masked>false</masked>
      <name>weekly_max_amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5b7246c6-20d4-4512-b867-71d4f751993a</id>
      <masked>false</masked>
      <name>monthly_max_amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>248a709f-9101-4d46-b8d7-f49c20aabba4</id>
      <masked>false</masked>
      <name>yearly_max_amount</name>
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
 
println(&quot;SetCardLimitAndRestriction request body: \n&quot;+ request.getBodyContent().getText())

 
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
println(&quot;SetCardLimitAndRestriction response body\n&quot;+json.toString(4)); // Print it with specified indentation
 

 </verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
