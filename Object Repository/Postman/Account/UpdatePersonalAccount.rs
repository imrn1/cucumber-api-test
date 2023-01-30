<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdatePersonalAccount</name>
   <tag></tag>
   <elementGuidId>bdeebda9-9928-4811-ac76-6222043a2702</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;account_id\&quot; : \&quot;${account_id}\&quot;,\n  \&quot;id\&quot;: \&quot;${id}\&quot;,\n  \&quot;tenant_id\&quot;: \&quot;${tenant_id}\&quot;,\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;alias\&quot;: \&quot;${alias}\&quot;,\n  \&quot;kyc_level\&quot;: \&quot;${kyc_level}\&quot;,\n  \&quot;owner_user_id\&quot;: \&quot;${owner_user_id}\&quot;,\n  \&quot;contact_address_contact_first_name\&quot;: \&quot;${contact_address_contact_first_name}\&quot;,\n  \&quot;contact_address_contact_last_name\&quot;: \&quot;${contact_address_contact_last_name}\&quot;,\n  \&quot;contact_address_contact_email\&quot;: \&quot;${contact_address_contact_email}\&quot;,\n  \&quot;contact_address_contact_phone\&quot;: \&quot;${contact_address_contact_phone}\&quot;,\n  \&quot;contact_address_address_line1\&quot;: \&quot;${contact_address_address_line1}\&quot;,\n  \&quot;contact_address_address_line2\&quot;: \&quot;${contact_address_address_line2}\&quot;,\n  \&quot;contact_address_zip_postal_code\&quot;: \&quot;${contact_address_zip_postal_code}\&quot;,\n  \&quot;contact_address_state_province_code\&quot;: \&quot;${contact_address_state_province_code}\&quot;,\n  \&quot;contact_address_country_code\&quot;: \&quot;${contact_address_country_code}\&quot;,\n  \&quot;access_level_status_id\&quot;: \&quot;${access_level_status_id}\&quot;,\n  \&quot;created_date_utc\&quot;: \&quot;${created_date_utc}\&quot;,\n  \&quot;updated_date_utc\&quot;: \&quot;${updated_date_utc}\&quot;\n}&quot;,
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
      <webElementGuid>43830487-7d7f-485f-80c2-71d2c7ba3f46</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>32bc43c3-9adb-4805-a484-b4b49797e0e5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/UpdatePersonalAccount</restUrl>
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
      <id>39331b90-0dab-467c-ba99-3e3f00699e56</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>005701ac-6888-48e8-813f-c6560c064ae0</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>727d48f0-e2c7-461c-9361-80e5769af6d3</id>
      <masked>false</masked>
      <name>account_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3dd541ba-83d5-44f8-9032-73c3f4dc95c0</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>910a8a6f-b844-4aed-82d7-4742c0b74844</id>
      <masked>false</masked>
      <name>tenant_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>66e3da21-22e2-4e0f-8339-088d62cf9b6d</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bbdb6a7e-cc35-490d-a897-0c173fdf8d38</id>
      <masked>false</masked>
      <name>alias</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>13a743b7-6343-4a14-90fd-9a61a3e559ea</id>
      <masked>false</masked>
      <name>kyc_level</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>de0ac35e-c6dc-4993-bebd-b95a808de96d</id>
      <masked>false</masked>
      <name>owner_user_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c5bb53ae-e802-45ae-a053-5eff6a518ae6</id>
      <masked>false</masked>
      <name>contact_address_contact_first_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>75c2351e-d07c-4b00-97a4-69dd6cc8c0c4</id>
      <masked>false</masked>
      <name>contact_address_contact_last_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>83099776-8426-49ff-93e4-a0d093870559</id>
      <masked>false</masked>
      <name>contact_address_contact_email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6da114fc-8ff0-4e90-9ba5-e2eef085c940</id>
      <masked>false</masked>
      <name>contact_address_contact_phone</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>53912c21-8c9a-4a85-9908-d5b36714c34c</id>
      <masked>false</masked>
      <name>contact_address_address_line1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>646ceefd-8b31-4633-b9ee-35406619c8ac</id>
      <masked>false</masked>
      <name>contact_address_address_line2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2ea0f436-9276-4226-bff2-a67fdec6e07a</id>
      <masked>false</masked>
      <name>contact_address_zip_postal_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e2ba1587-32fc-4bb2-9578-2d9d86b9974c</id>
      <masked>false</masked>
      <name>contact_address_state_province_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>25e6d107-dd01-4335-8a61-e3be3ba92a8b</id>
      <masked>false</masked>
      <name>contact_address_country_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>db66199a-6e2c-4ded-b7c3-66a15f91ba4e</id>
      <masked>false</masked>
      <name>access_level_status_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>50bc1c53-84f1-4606-87d9-3ec973959bed</id>
      <masked>false</masked>
      <name>created_date_utc</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a2910dc3-4d19-4bdb-b44a-97633cbbef0a</id>
      <masked>false</masked>
      <name>updated_date_utc</name>
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


println(&quot;UpdatePersonalAccount request body: \n&quot;+ request.getBodyContent().getText())

if(WS.getResponseStatusCode(response) == 400) {
	WS.verifyResponseStatusCode(response, 400)
}
if(WS.getResponseStatusCode(response) == 200) {
	WS.verifyResponseStatusCode(response, 200)
}

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;UpdatePersonalAccount response body\n&quot;+json.toString(4)); // Print it with specified indentation
 </verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
