<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UnassignUserFromWallet</name>
   <tag></tag>
   <elementGuidId>7771a9be-6d7c-43e2-a391-caee9b5fabf0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;tenant_id\&quot;: \&quot;${tenant_id}\&quot;,\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;account_type\&quot;: \&quot;${account_type}\&quot;,\n  \&quot;user_number\&quot;: \&quot;${user_number}\&quot;,\n  \&quot;wallet_number\&quot;: \&quot;${wallet_number}\&quot;\n}&quot;,
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
      <webElementGuid>2cd14fca-28e3-4ef1-b884-3bb29bc44bf7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>ccd97cf9-30e4-4231-b07a-c9efb452dc93</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/UnassignUserFromWallet</restUrl>
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
      <id>fe04024a-497b-41b2-9f20-a2be8b617f55</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>c047c8b4-baf0-4726-bf10-65a3d11e22a9</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>040105d6-55f2-4ef9-a5ff-21baa14db559</id>
      <masked>false</masked>
      <name>tenant_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c586df9f-978a-4eb1-873c-a6a5b5ede291</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7ea1a3e9-e348-4d6a-92d9-a6fac9e9a3ce</id>
      <masked>false</masked>
      <name>account_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>04250d90-8610-4e3f-a26b-b83ee989f4a0</id>
      <masked>false</masked>
      <name>user_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>68a12dd3-06c7-496b-84e5-a1e5d0e56f40</id>
      <masked>false</masked>
      <name>wallet_number</name>
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

println(&quot;UnassignUserFromWallet request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyElementPropertyValue(response, 'status', 0)

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;UnassignUserFromWallet response body\n&quot;+json.toString(4)); // Print it with specified indentation
 </verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
