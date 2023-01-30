<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpgradePersonalAccountKycLevelToContracted</name>
   <tag></tag>
   <elementGuidId>b2c4f454-3cf1-4ea5-a8e6-413a15596311</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;tenant_id\&quot;: \&quot;${tenant_id}\&quot;,\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;account_type\&quot;: \&quot;${account_type}\&quot;,\n  \&quot;kyc_level\&quot;: \&quot;${kyc_level}\&quot;\n}&quot;,
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
      <webElementGuid>ea42f152-957a-4d78-8ae9-2987b8fa1bd5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>f3df2ea6-dc12-421e-b735-e275dc4be9c0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/UpgradePersonalAccountKycLevelToContracted</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>c3869b65-2f73-4d21-b326-32242660c45b</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>bcd8e926-2009-44b5-84f9-e6109661e8bb</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>35acf1c8-cb61-4c89-bdb7-35738cfe22f2</id>
      <masked>false</masked>
      <name>tenant_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2b0b1cd1-b431-4b16-b05b-0930669a47ba</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ada46069-da0f-4e31-816f-9d74a7dfd602</id>
      <masked>false</masked>
      <name>account_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f00dfb82-34bc-49c4-b47f-d2cdee1dd968</id>
      <masked>false</masked>
      <name>kyc_level</name>
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

println(&quot;UpgradePersonalAccountKycLevelToContracted request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText());
println(&quot;UpgradePersonalAccountKycLevelToContracted response body\n&quot;+json.toString(4));










</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
