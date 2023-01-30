<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ListByFilter</name>
   <tag></tag>
   <elementGuidId>af41ff34-8382-4e07-ac6d-7d59a7d778b0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\n{\n  \&quot;tenant_id\&quot;: ${tenant_id},\n  \&quot;account_type_id\&quot;: ${account_type_id},\n  \&quot;account_id\&quot;: ${account_id},\n  \&quot;name\&quot;: ${name},\n  \&quot;account_number\&quot;: ${account_number},\n  \&quot;currency_code\&quot;: ${currency_code},\n  \&quot;kyc_level_code\&quot;: ${kyc_level_code},\n  \&quot;access_level_status_ids\&quot;: ${access_level_status_ids},\n  \&quot;start_date\&quot;: ${start_date},\n  \&quot;end_date\&quot;: ${end_date},\n  \&quot;is_deleted\&quot;: ${is_deleted},\n  \&quot;page_size\&quot;: ${page_size},\n  \&quot;page_index\&quot;: ${page_index},\n  \&quot;order_column\&quot;: ${order_column},\n  \&quot;order_by\&quot;: ${order_by},\n  \&quot;min\&quot;: ${min},\n  \&quot;max\&quot;: ${max},\n  \&quot;wallet_number\&quot;: ${wallet_number},\n  \&quot;first_name\&quot;: ${first_name},\n  \&quot;last_name\&quot;: ${last_name},\n  \&quot;national_id\&quot;: ${national_id}\n}\n\n&quot;,
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
      <webElementGuid>9b734208-04a7-447d-b161-cad335b122b5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>12eb9591-919a-47b7-b69a-4f65a533edfa</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Wallet/ListByFilter</restUrl>
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
      <id>94b87b77-5ff2-47fb-90bd-1b6ce5dcd09e</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>e42ab524-8be5-4064-9684-7a9660213471</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>7d458658-4b01-46b6-a117-d52cef570668</id>
      <masked>false</masked>
      <name>tenant_id</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>a8b77ea6-d8f7-42aa-92e6-da09d433c5c5</id>
      <masked>false</masked>
      <name>account_type_id</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>511030bd-ce03-4a2d-be5e-af58e94a7b1d</id>
      <masked>false</masked>
      <name>account_id</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>b9090438-969a-4c5a-909d-917d2183b6cf</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>05d75c84-ac91-4995-bede-729b2048db09</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>8d581705-c453-49b6-9e42-7f058ee59310</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>c04d94f1-5885-491b-a06e-be114208eb2d</id>
      <masked>false</masked>
      <name>kyc_level_code</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>11920af1-2060-471c-a899-4f7832dc86c1</id>
      <masked>false</masked>
      <name>access_level_status_ids</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>8ff76401-117b-49aa-9f50-d35f497e4090</id>
      <masked>false</masked>
      <name>start_date</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>f8ef870a-779a-4b51-8a2a-c47d4f03d4c5</id>
      <masked>false</masked>
      <name>end_date</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>6bc30a88-af7c-413d-94c0-9c5f4e72bd16</id>
      <masked>false</masked>
      <name>is_deleted</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>a36f9a97-e8ba-4ccc-a791-73d1767adcf3</id>
      <masked>false</masked>
      <name>page_size</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>d83922c0-3474-4716-816f-dab9d3ee293d</id>
      <masked>false</masked>
      <name>page_index</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>bb79cf43-2d5c-441d-b0b2-86f9d59990e5</id>
      <masked>false</masked>
      <name>order_column</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>278c06cc-050d-48f2-98d2-5c1594ded64a</id>
      <masked>false</masked>
      <name>order_by</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>054b744f-c120-4aef-938d-4825781abc0f</id>
      <masked>false</masked>
      <name>min</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>682e8ab0-9f2d-40e7-9782-afeec846dbf7</id>
      <masked>false</masked>
      <name>max</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>f11199ab-0fb3-4451-a988-762819fb01bd</id>
      <masked>false</masked>
      <name>wallet_number</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>e43f3bd4-a290-4af8-ab22-179043a5fb44</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>562b2a5a-51e0-4952-8f70-0c5220a50187</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>2a6ffa0b-f798-4b4e-9db7-1bdfbf99f1ba</id>
      <masked>false</masked>
      <name>national_id</name>
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


println(&quot;Wallet_ListByFilter request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)
  
import org.json.JSONObject;

JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;Wallet_ListByFilter response body\n&quot;+json.toString(4));



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
