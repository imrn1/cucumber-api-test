<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BusinessAccountSubmitKYCForm</name>
   <tag></tag>
   <elementGuidId>916f2322-9347-4c12-8ed0-2e1bf83a26fe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot; {\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;first_name\&quot;: \&quot;${first_name}\&quot;,\n  \&quot;last_name\&quot;: \&quot;${last_name}\&quot;,\n  \&quot;national_id\&quot;: \&quot;${national_id}\&quot;,\n  \&quot;birth_year\&quot;: \&quot;${birth_year}\&quot;,\n  \&quot;birth_month\&quot;: \&quot;${birth_month}\&quot;,\n  \&quot;birth_day\&quot;: \&quot;${birth_day}\&quot;,\n  \&quot;national_document_type_id\&quot;: \&quot;${national_document_type_id}\&quot;,\n  \&quot;country_code\&quot;: \&quot;${country_code}\&quot;,\n   \n   \n  \&quot;sector_id\&quot;: \&quot;${sector_id}\&quot;,\n  \&quot;address\&quot;: \&quot;${address}\&quot;,\n  \&quot;province_id\&quot;: \&quot;${province_id}\&quot;,\n  \&quot;city\&quot;: \&quot;${city}\&quot;\n}\n&quot;,
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
      <webElementGuid>a9efe592-96f2-458b-9ac3-d07db48c32ec</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>3f2dbad8-c4e3-40b2-a785-3e943904d98e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/BusinessAccountSubmitKYCForm</restUrl>
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
      <id>b01b0c8a-3478-44f6-ae1f-6f9bfcabe5a1</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>efbe6e18-b52b-4cd0-8fdd-6798ce680339</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>834b4410-abe2-48fb-999f-c49d15e7bacd</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d4df60b2-0a42-4e3c-bc88-801cbecb0a6a</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0ca4f198-ba54-4616-a97c-f18091f724c2</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8a4ee746-12a2-4ad9-b8a8-3418fe1d9e6f</id>
      <masked>false</masked>
      <name>national_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>aae6c77c-b0b8-4a41-bacf-6ed84c997d17</id>
      <masked>false</masked>
      <name>national_document_type_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9e8bb589-ec80-4a90-b128-089a66c9ba87</id>
      <masked>false</masked>
      <name>country_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>97115c51-f695-439c-8e4d-1baa5ca15a5b</id>
      <masked>false</masked>
      <name>birth_day</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>566b8318-3d88-476e-9dd9-989639a5e090</id>
      <masked>false</masked>
      <name>birth_month</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9c51343c-973b-4df0-9622-389d3676b8e1</id>
      <masked>false</masked>
      <name>birth_year</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>813a17b4-6a29-4f67-beff-1f86d6a43786</id>
      <masked>false</masked>
      <name>sector_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8b4e093e-3609-4b77-903b-241df75d956b</id>
      <masked>false</masked>
      <name>address</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>16301b7a-b06c-496d-8aa6-9565ba63d083</id>
      <masked>false</masked>
      <name>province_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4e93d6e4-3d6e-4eda-9423-41730a80531c</id>
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

println(&quot;BusinessAccountSubmitKYCForm request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText());
println(&quot;BusinessAccountSubmitKYCForm response body\n&quot;+json.toString(4));
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
