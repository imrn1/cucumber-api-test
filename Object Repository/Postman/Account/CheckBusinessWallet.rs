<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CheckBusinessWallet</name>
   <tag></tag>
   <elementGuidId>1c49ca2c-11a1-4a22-a2b4-d258eecdc913</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\n{\n  \&quot;account_number\&quot;: ${account_number},\n  \&quot;currency_code\&quot;: ${currency_code}\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>ed28a9ab-92fb-4e56-a14a-8cf38a01db81</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>284d1ec5-b62e-4752-aa8f-9077cc0365b8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/CheckBusinessWallet</restUrl>
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
      <id>54910a84-d0f7-4cf5-be7c-60bb67343532</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>b9814254-599d-4328-93f6-5fd4cfc06de4</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b3cd13e0-55ef-407a-8fbe-6bbe01e1727f</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1efed295-24db-441e-829f-74a3e8c418a2</id>
      <masked>false</masked>
      <name>currency_code</name>
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

println(&quot;CheckBusinessWallet request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;CheckBusinessWallet response body\n&quot;+json.toString(4)); // Print it with specified indentation
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
