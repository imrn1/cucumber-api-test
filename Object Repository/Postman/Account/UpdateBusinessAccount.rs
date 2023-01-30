<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateBusinessAccount</name>
   <tag></tag>
   <elementGuidId>f9c74983-01a6-400b-9f32-71bae1043d4d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;name\&quot;: \&quot;${name}\&quot;,\n  \&quot;alias\&quot;: \&quot;${alias}\&quot;,\n  \&quot;tax_office\&quot;: \&quot;${tax_office}\&quot;,\n  \&quot;tax_number\&quot;: \&quot;${tax_number}\&quot;,\n  \n  \&quot;contact_address\&quot;: {\n    \&quot;first_name\&quot;: \&quot;${first_name}\&quot;,\n    \&quot;last_name\&quot;: \&quot;${last_name}\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;address_line1\&quot;: \&quot;${address_line1}\&quot;,\n    \&quot;address_line2\&quot;: \&quot;${address_line2}\&quot;,\n    \&quot;zip_postal_code\&quot;: \&quot;${zip_postal_code}\&quot;,\n    \&quot;phone_number\&quot;: \&quot;${phone_number}\&quot;,\n    \&quot;state_province_code\&quot;: \&quot;${state_province_code}\&quot;,\n    \&quot;country_code\&quot;: \&quot;${country_code}\&quot;\n  }\n}&quot;,
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
      <webElementGuid>d94c8d36-3dce-42bc-8d91-dec77afba2e5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>e5b65aea-e02c-457a-9283-af84452fea23</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/UpdateBusinessAccount</restUrl>
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
      <id>b6dfa2ae-4617-43b1-926c-84cdb71325bd</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>22bbdb82-d551-4dc7-a98d-4f0086f255a6</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5da653ab-ee9b-4b01-8b38-a6ba7bdae077</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>70dc75e3-6850-4d7f-870e-22314b9b0386</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>466183d0-2f95-490e-a0fb-345f41f9c799</id>
      <masked>false</masked>
      <name>alias</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8ded6fb5-8bf3-44d7-8ea7-02a7d8f2ce6b</id>
      <masked>false</masked>
      <name>tax_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>eb46702c-d691-427c-bed3-b930bb6e1dd2</id>
      <masked>false</masked>
      <name>tax_office</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e474c093-493e-4026-9504-3692aa83469e</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ca820227-d2ac-4277-9bec-9de22693133a</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>99f3d3b7-d156-4ab9-84f9-9766dddcd016</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ed9eaf7b-e81c-4e72-ad23-0a68124819de</id>
      <masked>false</masked>
      <name>address_line1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5aab351c-9c48-4cd8-b29f-ceabb5115866</id>
      <masked>false</masked>
      <name>address_line2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8ea19a85-da6c-4716-93b9-8cbf46efa8e5</id>
      <masked>false</masked>
      <name>zip_postal_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>588f7d9a-9146-4754-8770-ab24a87bacee</id>
      <masked>false</masked>
      <name>phone_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>dc2187d2-ec41-42d6-9824-f38838cea9db</id>
      <masked>false</masked>
      <name>state_province_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ebf21892-eb56-43a6-84c3-f7ea53bc3fe7</id>
      <masked>false</masked>
      <name>country_code</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
