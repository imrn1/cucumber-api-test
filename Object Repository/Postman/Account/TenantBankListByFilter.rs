<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TenantBankListByFilter</name>
   <tag></tag>
   <elementGuidId>e8466da9-e86c-4d2a-91d5-2acef601a365</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;tenant_id\&quot;: \&quot;${tenant_id}\&quot;,\n    \&quot;draw\&quot;: \&quot;${draw}\&quot;,\n    \&quot;columns\&quot;: [\n        {\n        \&quot;data\&quot;: \&quot;${data}\&quot;,\n        \&quot;name\&quot;: \&quot;${name}\&quot;,\n        \&quot;searchable\&quot;: \&quot;${searchable}\&quot;,\n        \&quot;orderable\&quot;: \&quot;${orderable}\&quot;,\n        \&quot;search\&quot;: {\n            \&quot;value\&quot;: \&quot;${value}\&quot;,\n            \&quot;is_regex\&quot;: \&quot;${is_regex}\&quot;\n        }\n        }\n    ],\n    \&quot;order\&quot;: [\n        {\n        \&quot;column\&quot;: \&quot;${column}\&quot;,\n        \&quot;dir\&quot;: \&quot;${dir}\&quot;\n        }\n    ],\n    \&quot;start\&quot;: \&quot;${start}\&quot;,\n    \&quot;length\&quot;: \&quot;${length}\&quot;,\n    \&quot;search\&quot;: {\n        \&quot;value\&quot;: \&quot;${value1}\&quot;,\n        \&quot;is_regex\&quot;: \&quot;${is_regex1}\&quot;\n    },\n  \n  \&quot;page_size\&quot;: \&quot;${page_size}\&quot;,\n  \&quot;page_index\&quot;: \&quot;${page_index}\&quot;,\n  \&quot;order_column\&quot;: \&quot;${order_column}\&quot;,\n  \&quot;order_by\&quot;: \&quot;${order_by}\&quot;\n}\n&quot;,
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
      <webElementGuid>4a916b0a-515f-440c-9529-891e398a07aa</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>d5521d62-81fa-471e-a39e-7eb8dbc9963d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/TenantBankListByFilter</restUrl>
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
      <id>51288ef1-d23c-4485-9a4b-c1d1cb8330a3</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>d7bb779c-1792-43e5-9044-8724ae906e74</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ffcfbf38-6eae-47c0-89c8-9e385ecf2e9d</id>
      <masked>false</masked>
      <name>tenant_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>351bde22-617a-4297-a199-46c221498de8</id>
      <masked>false</masked>
      <name>draw</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d89e81b8-ef18-4c71-ba1f-ed0d1cf32f1e</id>
      <masked>false</masked>
      <name>data</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>810baea5-b4a8-4b9b-8846-fa9fd85184d9</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>197bb7d6-7512-4c4d-afc2-3ed6f2e61483</id>
      <masked>false</masked>
      <name>searchable</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>05de3aab-0539-4b63-b862-d38de7377b18</id>
      <masked>false</masked>
      <name>orderable</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f13876e1-eb54-456b-a687-1a39f50a4ada</id>
      <masked>false</masked>
      <name>value</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>15c0ee8b-e0ff-4863-b2da-1b9f61a2fe7f</id>
      <masked>false</masked>
      <name>is_regex</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1bf77ada-c84e-4e23-b8bf-2a886957ef0a</id>
      <masked>false</masked>
      <name>column</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9697bfd7-cc55-414d-9b59-24222fab6575</id>
      <masked>false</masked>
      <name>dir</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>560cbfb2-97bf-4ae7-ad12-3dc5de5bc3df</id>
      <masked>false</masked>
      <name>start</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ec0768fa-ba1b-4cc2-bb59-35e4bb1580c8</id>
      <masked>false</masked>
      <name>length</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e8566a42-3946-4a49-86d5-b12d5ad84154</id>
      <masked>false</masked>
      <name>value1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7f281bf3-f948-477d-95ae-139a616c6739</id>
      <masked>false</masked>
      <name>is_regex1</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>1330b9c8-21ef-48a9-b8b9-8ac2c79e6216</id>
      <masked>false</masked>
      <name>page_size</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>d876622b-caf9-4e68-b2e4-73296b320fc9</id>
      <masked>false</masked>
      <name>page_index</name>
   </variables>
   <variables>
      <defaultValue>'Id'</defaultValue>
      <description></description>
      <id>6d225b8c-80c4-42ab-8896-caa5570c5f55</id>
      <masked>false</masked>
      <name>order_column</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d2657e91-9cce-48b0-b800-6049f76680be</id>
      <masked>false</masked>
      <name>order_by</name>
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

println(&quot;TenantBankListByFilter request body: \n&quot;+ request.getBodyContent().getText())

if(WS.getResponseStatusCode(response) == 400) {
	WS.verifyResponseStatusCode(response, 400)
}
if(WS.getResponseStatusCode(response) == 200) {
	WS.verifyResponseStatusCode(response, 200)
}

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;TenantBankListByFilter response body\n&quot;+json.toString(4)); // Print it with specified indentation
 


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
