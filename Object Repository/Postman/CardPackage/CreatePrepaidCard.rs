<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreatePrepaidCard</name>
   <tag></tag>
   <elementGuidId>f700bbee-6732-410c-8c5b-9a86146c61e9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;wallet_number\&quot;: \&quot;${wallet_number}\&quot;,\n  \&quot;account_type\&quot;: \&quot;${account_type}\&quot;,\n  \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;, \n  \&quot;product_code\&quot;: \&quot;${product_code}\&quot;,\n  \&quot;delivery_address\&quot;: {\n    \&quot;city\&quot;: \&quot;${city}\&quot;,\n    \&quot;city_code\&quot;: \&quot;${city_code}\&quot;,\n    \&quot;address_type\&quot;: \&quot;${address_type}\&quot;,\n    \&quot;country_code\&quot;: \&quot;${country_code}\&quot;,\n    \&quot;district\&quot;: \&quot;${district}\&quot;,\n    \&quot;Address1\&quot;: \&quot;${Address1}\&quot;,\n    \&quot;Address2\&quot;: \&quot;${Address2}\&quot;,\n    \&quot;Address3\&quot;: \&quot;${Address3}\&quot;,\n    \&quot;Address4\&quot;:\&quot;${Address4}\&quot; ,\n    \&quot;address_code\&quot;: \&quot;${address_code}\&quot;,\n    \&quot;zip_code\&quot;: \&quot;${zip_code}\&quot;\n  }\n}&quot;,
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
      <webElementGuid>ca3ac092-eb55-4c7e-bca6-09c346f8de40</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>2468cc63-c782-40c8-ae50-4d83e5db8d91</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}​/v1​/cp​/Issuing​/CreatePrepaidCard</restUrl>
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
      <id>375c569f-816b-4d92-939f-a525489c81fe</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>5abc9e45-aa33-4615-9df1-0cd94bdf61c3</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ef61ff29-81df-492f-9856-21007f150a24</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ef33b6b4-6ab5-4476-b8d7-9722a88f59e2</id>
      <masked>false</masked>
      <name>wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>010979d2-7f87-4b11-b1bd-79838f3e5049</id>
      <masked>false</masked>
      <name>account_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4acca3dd-395b-47de-a7cc-f9b91d6d7679</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3f3ad611-7f08-415e-bd5d-34f91a7a30be</id>
      <masked>false</masked>
      <name>product_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1ad2d470-af84-460b-95db-bbfcbe6a34fc</id>
      <masked>false</masked>
      <name>city</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>96348837-f6e3-47dc-a016-6d1cad4283ed</id>
      <masked>false</masked>
      <name>city_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>42b2ed74-79a5-4cc1-a1c4-78c6fc6378e7</id>
      <masked>false</masked>
      <name>address_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0e23aab9-e2b7-4999-b18f-145b4763c631</id>
      <masked>false</masked>
      <name>country_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2312d3d5-e88c-4d21-a69e-4b5833bfc5a5</id>
      <masked>false</masked>
      <name>district</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>06934906-a027-4fcd-9eb8-5cd2472f475f</id>
      <masked>false</masked>
      <name>Address4</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5ab3e74a-8ed0-4511-82ee-bdb3174c215e</id>
      <masked>false</masked>
      <name>Address1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>039c300b-79b5-41d2-a713-bebc037372b1</id>
      <masked>false</masked>
      <name>Address2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9d628d55-63f2-42f3-bccd-9ad2f104ddfb</id>
      <masked>false</masked>
      <name>Address3</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5bbfbdc6-068d-4829-a148-b45dbe0fab81</id>
      <masked>false</masked>
      <name>address_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4631788b-e6a1-48b3-9139-a5d7d55f7f5a</id>
      <masked>false</masked>
      <name>zip_code</name>
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


println(&quot;CreatePrepaidCard request body: \n&quot;+ request.getBodyContent().getText())

 
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
println(&quot;CreatePrepaidCard response body\n&quot;+json.toString(4)); // Print it with specified indentation
 
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
