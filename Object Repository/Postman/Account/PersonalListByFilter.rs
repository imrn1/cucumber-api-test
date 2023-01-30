<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PersonalListByFilter</name>
   <tag></tag>
   <elementGuidId>a6905335-3956-4912-bd91-a5278d375efa</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \n  \&quot;id\&quot;: \&quot;${id}\&quot;,  \n  \&quot;tenant_id\&quot;: \&quot;${tenant_id}\&quot;,\n  \&quot;account_id\&quot;:\&quot;${account_id}\&quot;,\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \n  \&quot;first_name\&quot;: \&quot;${first_name}\&quot;,\n  \&quot;last_name\&quot;: \&quot;${last_name}\&quot;,\n  \&quot;end_date\&quot;:\&quot;${end_date}\&quot;,      \n  \&quot;start_date\&quot;: \&quot;${start_date}\&quot;,\n  \&quot;access_level_status_id\&quot;: \&quot;${access_level_status_id}\&quot;,   \n  \&quot;kyc_level\&quot;: \&quot;${kyc_level}\&quot;,\n  \&quot;phone_number\&quot;: \&quot;${phone_number}\&quot;,\n  \&quot;email\&quot;: \&quot;${email}\&quot;,    \n  \&quot;phone_country_code\&quot;: \&quot;${phone_country_code}\&quot;,\n  \&quot;national_id\&quot;: \&quot;${national_id}\&quot;,\n  \&quot;page_size\&quot;: \&quot;${page_size}\&quot;,\n  \&quot;page_index\&quot;: \&quot;${page_index}\&quot;,\n  \&quot;order_column\&quot;:\&quot;${order_column}\&quot;,\n  \&quot;order_by\&quot;: \&quot;${order_by}\&quot;\n}\n\n&quot;,
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
      <webElementGuid>fe5aceb9-f200-4529-9ed1-9c9eb6ff33a7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>5b48ca14-8f9c-42b5-a62b-2a608bf8f869</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/PersonalListByFilter</restUrl>
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
      <id>a4f37f7e-31b5-4d57-a27e-18a74ec6b843</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>9f9c360f-1301-42ea-91ba-d0a4f0658aca</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c1dfccb8-6cb7-4802-b12d-71e3fa77a19d</id>
      <masked>false</masked>
      <name>national_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3b3a43d9-9b19-4754-adf6-8db1f7975e03</id>
      <masked>false</masked>
      <name>page_size</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1d99695b-a7ec-4061-8a15-344cc8473fb9</id>
      <masked>false</masked>
      <name>order_by</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1b2b1307-3232-420d-ab9c-deb2757ccc4c</id>
      <masked>false</masked>
      <name>page_index</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3a56636e-8028-4898-b08b-4e4e196425c1</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>829bbf52-f846-4c6f-9aa1-fe9c667ff498</id>
      <masked>false</masked>
      <name>tenant_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8d4b86e4-870b-4985-80e8-da23e4113239</id>
      <masked>false</masked>
      <name>order_column</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d6173275-b319-40e3-bc6c-fd1944eb2718</id>
      <masked>false</masked>
      <name>account_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0eb99d5a-27ad-4e13-bd12-70ee08a91f25</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bd902a71-7218-42bd-ba88-c7eb77ea25e6</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9ee25f93-8d89-402f-a70a-52f7fc3a3e1e</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c07a252e-39f8-4811-82b4-f257825287a0</id>
      <masked>false</masked>
      <name>end_date</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5ac25a6c-75d9-4735-a368-39a99524a405</id>
      <masked>false</masked>
      <name>start_date</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3b07d215-f8a2-4d98-ac66-68eee6a2f748</id>
      <masked>false</masked>
      <name>access_level_status_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a9591d29-dfff-4803-aa29-2f873f47eb63</id>
      <masked>false</masked>
      <name>kyc_level</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>33ebca95-19c4-4ac9-bace-7a9e1278e0e9</id>
      <masked>false</masked>
      <name>phone_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1c70a18b-f549-4712-b873-1d898b2aef1f</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5daa512c-e826-4989-9a4e-0a595ec7f649</id>
      <masked>false</masked>
      <name>phone_country_code</name>
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


println(&quot;PersonalListByFilter request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)


import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;PersonalListByFilter response body\n&quot;+json.toString(4)); // Print it with specified indentation

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
