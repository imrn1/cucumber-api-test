<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SubmitKYCForm</name>
   <tag></tag>
   <elementGuidId>97888547-5d8c-45be-965f-a8a5c7e954d1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;first_name\&quot;:\&quot;${first_name}\&quot;,\n    \&quot;last_name\&quot;:\&quot;${last_name}\&quot;,\n  \t\&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n    \&quot;national_id\&quot;:\&quot;${national_id}\&quot;,\n    \&quot;birth_year\&quot;:\&quot;${birth_year}\&quot;,\n    \&quot;birth_day\&quot;: \&quot;${birth_day}\&quot;,\n    \&quot;birth_month\&quot;: \&quot;${birth_month}\&quot;,\n  \n    \&quot;national_document_type_id\&quot;: \&quot;${national_document_type_id}\&quot;,\n  \t\&quot;country_code\&quot;: \&quot;${country_code}\&quot;,\n  \n    \&quot;sector_id\&quot;:\&quot;${sector_id}\&quot;,\n    \&quot;address\&quot;: \&quot;${address}\&quot;,\n    \&quot;province_id\&quot;: \&quot;${province_id}\&quot;,\n    \&quot;city\&quot;: \&quot;${city}\&quot;\n}\n\n\n\n&quot;,
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
      <webElementGuid>180fa60a-4feb-414c-8f27-e496f2b876b5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>0de005f3-6028-4adb-b365-0b97a97001f0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/SubmitKYCForm</restUrl>
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
      <id>bf0fd80b-b1c7-47eb-862c-3f4266d5e063</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>506181d4-e8e2-4499-84d1-f57bd7e27799</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5f6c04f5-b7ce-4821-b9e9-e588ccb895af</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>048d1037-f48f-4e3a-bf2a-fc81d17eb200</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>88bd5501-c70a-4531-a442-21f4eb1c05ee</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>41fd68b9-d033-444c-9e69-393b0e02b8e9</id>
      <masked>false</masked>
      <name>national_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9dd4bfda-3ec9-4e6b-a81c-7de0d05f8658</id>
      <masked>false</masked>
      <name>birth_year</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8e807c84-6f6e-4014-aa18-d0983e00a79b</id>
      <masked>false</masked>
      <name>birth_day</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f4492c57-807f-4591-940c-33e13b3b1437</id>
      <masked>false</masked>
      <name>birth_month</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>6f2b9460-382d-40a3-997d-029dfee4b09b</id>
      <masked>false</masked>
      <name>national_document_type_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>35658794-0dad-45db-9c35-c62c5ee4314d</id>
      <masked>false</masked>
      <name>country_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>04ef225d-d5eb-442f-b6bf-1fe713793fc5</id>
      <masked>false</masked>
      <name>sector_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>762bed3f-f642-418a-bec9-c02bdfd28e39</id>
      <masked>false</masked>
      <name>address</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ee953c16-ef9d-4beb-8a35-04d5cbc53b59</id>
      <masked>false</masked>
      <name>province_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e3d7539a-8864-44fa-9f61-172a48b905c8</id>
      <masked>false</masked>
      <name>city</name>
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

println(&quot;SubmitKYCForm request body: \n&quot;+ request.getBodyContent().getText())

if(WS.getResponseStatusCode(response) == 400) {
	WS.verifyResponseStatusCode(response, 400)
}
if(WS.getResponseStatusCode(response) == 200) {
	WS.verifyResponseStatusCode(response, 200)
}

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;SubmitKYCForm response body\n&quot;+json.toString(4)); // Print it with specified indentation
 
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
