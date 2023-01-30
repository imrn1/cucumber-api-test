<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddUserToAccount</name>
   <tag></tag>
   <elementGuidId>042a2af8-3733-4ce2-9334-072254713afb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;tenant_id\&quot;: \&quot;${tenant_id}\&quot;,\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;account_type\&quot;: \&quot;${account_type}\&quot;,\n  \&quot;user_number\&quot;: \&quot;${user_number}\&quot;,\n  \&quot;email\&quot;: \&quot;${email}\&quot;,\n  \&quot;password\&quot;: \&quot;${password}\&quot;,\n  \&quot;first_name\&quot;: \&quot;${first_name}\&quot;,\n  \&quot;last_name\&quot;: \&quot;${last_name}\&quot;,\n  \&quot;phone_country_code\&quot;: \&quot;${phone_country_code}\&quot;,\n  \&quot;phone_number\&quot;: \&quot;${phone_number}\&quot;\n}&quot;,
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
      <webElementGuid>595c4d9e-2bf6-4fe4-9e94-516904e67367</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>a0debb54-2b97-46d6-94ad-ec46fcf98f21</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/AddUserToAccount</restUrl>
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
      <id>cc506981-a90b-4bba-8b5b-fa9fd31a5d64</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.account_number</defaultValue>
      <description></description>
      <id>6260c37d-4c34-44da-af71-b34c94c35c91</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.user_number</defaultValue>
      <description></description>
      <id>a141eb28-7b7a-4104-a8b1-73740c097046</id>
      <masked>false</masked>
      <name>user_number</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.first_name</defaultValue>
      <description></description>
      <id>245e33b9-ff68-48fe-98ae-83b918cd2b3d</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.last_name</defaultValue>
      <description></description>
      <id>1aadc961-a689-443f-a7d7-e3d837051984</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.phone_country_code</defaultValue>
      <description></description>
      <id>a75aef70-04df-4aff-841f-90ef2f3134bb</id>
      <masked>false</masked>
      <name>phone_country_code</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.phone_number</defaultValue>
      <description></description>
      <id>e6cec3de-6a80-4323-a8c9-6656d5881d42</id>
      <masked>false</masked>
      <name>phone_number</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>5aafaa12-b42b-44eb-ba20-68d2be888eb5</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tenant_id</defaultValue>
      <description></description>
      <id>8196b6af-075d-419b-acc8-3809c2efc9ed</id>
      <masked>false</masked>
      <name>tenant_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.account_type</defaultValue>
      <description></description>
      <id>e8911dbf-83e3-43cd-84db-356a52acd8e2</id>
      <masked>false</masked>
      <name>account_type</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.email</defaultValue>
      <description></description>
      <id>296e41e9-a530-482d-b01c-01161cee14a1</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.password</defaultValue>
      <description></description>
      <id>00074159-2cf7-4060-990e-0babdbbeef4c</id>
      <masked>false</masked>
      <name>password</name>
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


println(&quot;AddUserToAccount request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyElementPropertyValue(response, 'status', 0)

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;AddUserToAccount response body\n&quot;+json.toString(4)); // Print it with specified indentation


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
