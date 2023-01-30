<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CheckPersonalWallet</name>
   <tag></tag>
   <elementGuidId>57cb7d55-bc04-4d8e-a6bc-7021186a3236</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\n{\n  \&quot;phone_country_number\&quot;: ${phone_country_number}, \n  \&quot;phone_number\&quot;: ${phone_number},\n\n  \&quot;account_number\&quot;: ${account_number},\n  \&quot;currency_code\&quot;: ${currency_code}\n}&quot;,
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
      <webElementGuid>7e0adf13-b595-4b01-88ce-bc554065a239</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>425c1c3a-cad0-4a96-988b-f9334176c28f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/CheckPersonalWallet</restUrl>
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
      <id>8c72ec6f-96e1-46bc-b9ba-92a48e1c215e</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>abbe0f31-5fdf-4baf-9396-159257681e0d</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>35f83998-c2ff-43e3-a237-a17c7d913233</id>
      <masked>false</masked>
      <name>phone_country_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a0131722-fc48-45ba-a9c8-3a190f59b5ad</id>
      <masked>false</masked>
      <name>phone_number</name>
   </variables>
   <variables>
      <defaultValue>'TRY'</defaultValue>
      <description></description>
      <id>612c77a8-b643-442b-a193-7717fa11c619</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fc0d41ff-67eb-4fd1-b2a1-65f54ac5294c</id>
      <masked>false</masked>
      <name>account_number</name>
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

println(&quot;CheckPersonalWallet request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)
  
import org.json.JSONObject;

JSONObject json = new JSONObject(response.getBodyContent().getText());

println(&quot;CheckPersonalWallet response body\n&quot;+json.toString(4));
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
