<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>IDS_Server_Authentication_For_STS_API</name>
   <tag></tag>
   <elementGuidId>cc54a4f0-afb3-4f9a-9862-57e04cd483c8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;grant_type&quot;,
      &quot;value&quot;: &quot;client_credentials&quot;
    },
    {
      &quot;name&quot;: &quot;scope&quot;,
      &quot;value&quot;: &quot;sts_api&quot;
    },
    {
      &quot;name&quot;: &quot;client_id&quot;,
      &quot;value&quot;: &quot;${client_id}&quot;
    },
    {
      &quot;name&quot;: &quot;client_secret&quot;,
      &quot;value&quot;: &quot;${client_secret}&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
      <webElementGuid>a0c4a3ef-c7c2-424d-9439-a8f68d00d7ed</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${identityserver_url}/connect/token</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.identityserver_url</defaultValue>
      <description></description>
      <id>5f13df6b-5d75-43bc-bc04-5eefc2617985</id>
      <masked>false</masked>
      <name>identityserver_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.client_id</defaultValue>
      <description></description>
      <id>02a2f49b-1f11-4d7a-bfe0-cea422ebe219</id>
      <masked>false</masked>
      <name>client_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.client_secret</defaultValue>
      <description></description>
      <id>d98b9244-2cdf-4f3f-be00-59bc9663f5c7</id>
      <masked>false</masked>
      <name>client_secret</name>
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

//println(&quot;connect/token request body: &quot;+ request.getBodyContent().getText())

println(&quot;connect/token response: \n&quot;+response.getBodyContent().getText())


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
