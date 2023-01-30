<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateCardInquiry</name>
   <tag></tag>
   <elementGuidId>d1d68719-ecf2-490a-affb-8a1dc1ff1b86</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;user_number\&quot;: \&quot;${user_number}\&quot;,\n  \&quot;product_type\&quot;: \&quot;${product_type}\&quot;, \n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;wallet_number\&quot;: \&quot;${wallet_number}\&quot;,\n  \&quot;account_type\&quot;: \&quot;${account_type}\&quot;,\n  \&quot;product_code\&quot;: \&quot;${product_code}\&quot;\n}&quot;,
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
      <webElementGuid>6899d508-6043-4d75-a49d-54c2a0316b48</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>b22fd594-ec61-4307-89bf-9845f13414f7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/cp/Issuing/CreateCardInquiry</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>ad442179-ca8c-4f63-8882-b0fd5754bb3c</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>8c9dffff-e0e9-44f7-a27d-7bffed5d6445</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f1cadc18-e2f3-4df4-815c-c85eff642300</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>085b71ea-4cc5-416d-9e3e-c389f3d5d2c9</id>
      <masked>false</masked>
      <name>wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>902bf2b8-498f-4b0f-975e-dec1797bd908</id>
      <masked>false</masked>
      <name>account_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4579da51-d443-4af1-9753-95955b973239</id>
      <masked>false</masked>
      <name>product_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f3e31e39-7302-4809-a445-e64c62d54250</id>
      <masked>false</masked>
      <name>user_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>99a4d348-6bd1-484c-9ba5-a5c1465a6e03</id>
      <masked>false</masked>
      <name>product_type</name>
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
 

println(&quot;CreateCardInquiry request body: \n&quot;+ request.getBodyContent().getText())

 
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
println(&quot;CreateCardInquiry response body\n&quot;+json.toString(4)); // Print it with specified indentation
 </verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
