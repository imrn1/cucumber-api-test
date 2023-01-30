<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CheckQRPersonalToPersonalTransfer</name>
   <tag></tag>
   <elementGuidId>91215751-8d94-4f65-8265-5da1ce4b406a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;qr_code\&quot;: \&quot;${qr_code}\&quot;\n}&quot;,
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
      <webElementGuid>21174fdb-370d-4658-97b8-16ff8dd50229</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>75b0ca40-ad37-4fbe-842a-d97a2ca378a0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/CheckQRPersonalToPersonalTransfer</restUrl>
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
      <id>cbf5420a-8abe-4819-a540-c0e7fdf2dbe7</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>6a543510-1603-4072-86ed-eeded1f1baa2</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>256f2d36-086c-4a01-8223-af38d6980105</id>
      <masked>false</masked>
      <name>qr_code</name>
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


println(&quot;CheckQRPersonalToPersonalTransfer request body: \n&quot;+ request.getBodyContent().getText())


if(WS.getResponseStatusCode(response) == 400) {
	WS.verifyResponseStatusCode(response, 400)
}
if(WS.getResponseStatusCode(response) == 200) {
	WS.verifyResponseStatusCode(response, 200)
}

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;CheckQRPersonalToPersonalTransfer response body\n&quot;+json.toString(4));


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
